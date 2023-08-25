use crate::api::{
    issues::{self, IssueCategory},
    objects,
};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use log::info;
use uuid::Uuid;
use std::{option::Option, result::Result};
use tokio_postgres::NoTls;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

const STMT_GET_PRICING: &str = "SELECT price, period, description FROM pricing WHERE id = $1";
const STMT_GET_PRICING_ID_BY_OBJECT_TYPE: &str = "SELECT id FROM pricing WHERE object_type = $1";
const STMT_ADD_INVOICE: &str = "INSERT INTO invoice(object_type, object_name, start_time, end_time, price_id VALUES ($1, $2, $3, $4, $5)";
const STMT_GET_INVOICE_ID_BY_END_TIME: &str =
    "SELECT id FROM invoice WHERE object_type = $1 AND object_name = $2 AND end_time >= $3";
//const STMT_GET_OBJECT_ID: &str = "SELECT id FROM namespaced_objects WHERE object_type = $1 AND object_name = $2 AND cluster_name = $3 AND namespace = $4";
const STMT_CREATE_OBJECT_AND_RETURN_ID: &str = "INSERT INTO namespaced_objects (cluster_name, namespace_name, object_name, object_type) \
	VALUES ($1,$2,$3,$4) ON CONFLICT ON CONSTRAINT pkey_issues_objects DO UPDATE set cluster_name=$1 RETURNING id";
const STMT_ADD_OBJECT_ISSUE: &str = "INSERT INTO issues(object_id, category, details, severity, issue_tech_id, issue_message, reported_by, reported_at, last_seen_at, \
	linked_object_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";
const STMT_GET_NAMESPACED_OBJECTS_WITH_ISSUES_WITH_CATEGORY: &str = "SELECT id, object_type, object_name, namespace_name, cluster_name FROM namespaced_objects WHERE \
	(\
		id IN (SELECT object_id FROM issues WHERE category = $2) \
		OR \
		id IN (SELECT linked_object_id FROM issues WHERE category = $2) \
	) AND namespace_name = $1";
const STMT_GET_ISSUES_WITH_CATEGORY_FOR_NAMESPACE: &str = "SELECT object_id, category, details, severity, issue_tech_id, issue_message, reported_by, reported_at, last_seen_at, linked_object_id \
	FROM issues WHERE object_id IN (SELECT id FROM namespaced_objects WHERE namespace_name = $2) AND category = $1";

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(
        db_host: Option<String>,
        db_name: Option<String>,
        username: Option<String>,
        password: Option<String>,
        pool_size: usize,
    ) -> Result<Database, Error> {
        let db_config = Database::create_db_config(db_host, db_name, username, password).await;
        let db = Database {
            pool: Database::create_pool(db_config, pool_size).await?,
        };

        db.test_connection().await?;
        db.run_migrations().await?;
        Ok(db)
    }

    async fn create_db_config(
        db_host: Option<String>,
        db_name: Option<String>,
        username: Option<String>,
        password: Option<String>,
    ) -> tokio_postgres::Config {
        let mut cfg = tokio_postgres::Config::new();
        match db_host {
            Some(h) => {
                cfg.host(h.as_str());
            }
            None => {
                cfg.host("localhost");
            }
        };
        match username {
            Some(u) => {
                cfg.user(u.as_str());
            }
            None => {}
        };
        match password {
            Some(p) => {
                cfg.password(p.as_str());
            }
            None => {}
        };
        match db_name {
            Some(db) => {
                cfg.dbname(db.as_str());
            }
            None => {}
        };
        cfg
    }

    async fn create_pool(cfg: tokio_postgres::Config, pool_size: usize) -> Result<Pool, Error> {
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Verified,
        };

        let mgr = Manager::from_config(cfg, NoTls, mgr_config);
        let pool = Pool::builder(mgr).max_size(pool_size).build().unwrap();

        Ok(pool)
    }

    async fn test_connection(&self) -> Result<(), Error> {
        let client = self.pool.get().await?;
        let row = client.query_one("SELECT 1", &[]).await?;
        let one: i32 = row.get(0);
        if one != 1 {
            return Err("PostgreSQL connection test failure".into());
        }
        Ok(())
    }

    async fn run_migrations(&self) -> Result<(), Error> {
        info!("Running DB migrations...");
        let mut conn = self.pool.get().await.unwrap();
        let client = &mut **conn;
        let migration_report = embedded::migrations::runner().run_async(client).await?;

        for migration in migration_report.applied_migrations() {
            info!(
                "Migration Applied -  Name: {}, Version: {}",
                migration.name(),
                migration.version()
            );
        }

        Ok(())
    }

    // pub async fn get_object_id(
    //     &self,
    //     object_type: &str,
    //     object_name: &str,
    //     cluster_name: &str,
    //     namespace: &str,
    // ) -> Result<String, Error> {
    //     let conn = self.pool.get().await?;
    //     let row = conn
    //         .query_one(
    //             STMT_GET_OBJECT_ID,
    //             &[&object_type, &object_name, &cluster_name, &namespace],
    //         )
    //         .await?;
    //     let id: String = row.get("id");
    //     Ok(id)
    // }

	pub async fn record_namespaced_object(&self, object_type: &str, object_name: &str, cluster_name: &str, namespace: &str) -> Result<Uuid, Error> {
		let conn = self.pool.get().await?;
		let row = conn
            .query_one(
                STMT_CREATE_OBJECT_AND_RETURN_ID,
                &[&cluster_name, &namespace, &object_name, &object_type],
            )
            .await?;
        let id: Uuid = row.get(0);
        Ok(id)
	}

    pub async fn add_object_issue(&self, object_issue: issues::Issue) -> Result<(), Error> {
        let conn = self.pool.get().await?;
        conn.execute(
            STMT_ADD_OBJECT_ISSUE,
            &[
                &object_issue.object_id,
                &object_issue.category,
                &object_issue.details,
                &object_issue.severity,
                &object_issue.issue_tech_id,
                &object_issue.issue_message,
                &object_issue.reported_by,
                &object_issue.reported_at,
                &object_issue.last_seen_at,
                &object_issue.linked_object_id,
            ],
        )
        .await?;
        Ok(())
    }

    pub async fn get_issues_with_category_for_namespace(
        &self,
        category: IssueCategory,
        namespace: &str,
    ) -> Result<Vec<issues::Issue>, Error> {
        let conn = self.pool.get().await?;
        let rows = conn
            .query(
                STMT_GET_ISSUES_WITH_CATEGORY_FOR_NAMESPACE,
                &[&category, &namespace],
            )
            .await?;
        let mut issues: Vec<issues::Issue> = Vec::new();
        for row in rows {
            let issue = issues::Issue {
                object_id: row.get("object_id"),
                category: row.get("category"),
                details: row.get("details"),
                severity: row.get("severity"),
                issue_tech_id: row.get("issue_tech_id"),
                issue_message: row.get("issue_message"),
                reported_by: row.get("reported_by"),
                reported_at: row.get("reported_at"),
                last_seen_at: row.get("last_seen_at"),
                linked_object_id: row.get("linked_object_id"),
            };
            issues.push(issue);
        }
        Ok(issues)
    }

    pub async fn get_objects_with_issue_category_in_namespace(
        &self,
        category: IssueCategory,
        namespace: &str,
    ) -> Result<Vec<objects::NamespacedObject>, Error> {
        let conn = self.pool.get().await?;
        let rows = conn
            .query(
                STMT_GET_NAMESPACED_OBJECTS_WITH_ISSUES_WITH_CATEGORY,
                &[&namespace, &category],
            )
            .await?;
        let mut objects: Vec<objects::NamespacedObject> = Vec::new();
        for row in rows {
            let object = objects::NamespacedObject {
                id: row.get("id"),
                object_type: row.get("object_type"),
                object_name: row.get("object_name"),
                namespace: row.get("namespace"),
                cluster: row.get("cluster"),
            };
            objects.push(object);
        }
        Ok(objects)
    }
}

// For database migrations
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}
