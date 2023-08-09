use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use std::{
	option::Option,
	result::Result
};
use crate::api::objects;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

const STMT_GET_PRICING: &str = "SELECT price, period, description FROM pricing WHERE id = $1";
const STMT_GET_PRICING_ID_BY_OBJECT_TYPE: &str = "SELECT id FROM pricing WHERE object_type = $1";
const STMT_ADD_INVOICE: &str = "INSERT INTO invoice(object_type, object_name, start_time, end_time, price_id VALUES ($1, $2, $3, $4, $5)";
const STMT_GET_INVOICE_ID_BY_END_TIME: &str = "SELECT id FROM invoice WHERE object_type = $1 AND object_name = $2 AND end_time >= $3";
const STMT_ADD_NAMESPACED_OBJECT: &str = "INSERT INTO namespaced_object(object_type, object_name, namespace, cluster) VALUES ($1, $2, $3, $4)";
const STMT_GET_NAMESPACED_OBJECT_ID: &str = "SELECT id FROM namespaced_object WHERE object_type = $1 AND object_name = $2 AND namespace = $3 AND cluster = $4";
const STMT_ADD_OBJECT_ISSUE: &str = "INSERT INTO object_issue(object_id, category, details, severity, issue_tech_id, issue_message, reported_by, reported_at, last_seen_at, \
	linked_object_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";
const STMT_GET_OBJECT_ISSUES: &str = "SELECT object_id, category, details, severity, issue_tech_id, issue_message, reported_by, reported_at, last_seen_at, linked_object_id \
	FROM object_issue WHERE object_id IN (SELECT id FROM namespaced_object WHERE object_type = $1 AND object_name = $2 AND namespace = $3 AND cluster = $4)";
pub struct Database {
	pool: Pool
}

impl Database {
	pub async fn new(db_host: Option<String>, db_name: Option<String>, username: Option<String>, password: Option<String>, pool_size: usize) -> Result<Database, Error>{

		let db_config = Database::create_db_config(db_host, db_name, username, password).await;
		let db = Database {
			pool: Database::create_pool(db_config, pool_size).await?,
		};

		db.test_connection().await?;
		db.run_migrations().await?;
		Ok(db)
	}

	async fn create_db_config(db_host: Option<String>, db_name: Option<String>, username: Option<String>, password: Option<String>) -> tokio_postgres::Config {
		let mut cfg = tokio_postgres::Config::new();
		match db_host {
			Some(h) => { cfg.host(h.as_str()); },
			None => { cfg.host("localhost"); },
		};
		match username {
			Some(u) => { cfg.user(u.as_str()); },
			None => {},
		};
		match password {
			Some(p) => { cfg.password(p.as_str()); },
			None => {},
		};
		match db_name {
			Some(db) => { cfg.dbname(db.as_str()); },
			None => {},
		};
		cfg
	}

	async fn create_pool(cfg: tokio_postgres::Config, pool_size: usize) -> Result<Pool, Error> {
		let mgr_config = ManagerConfig {
			recycling_method: RecyclingMethod::Verified
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
		println!("Running DB migrations...");
		let mut conn = self.pool.get().await.unwrap();
		let client = &mut **conn;
		let migration_report = embedded::migrations::runner()
			.run_async(client)
			.await?;

		for migration in migration_report.applied_migrations() {
			println!(
				"Migration Applied -  Name: {}, Version: {}",
				migration.name(),
				migration.version()
			);
		}

		println!("DB migrations finished!");

		Ok(())
	}

	async fn add_namespaced_object(&self, ns_object: objects::NamespacedObject) -> Result<(), Error> {
		let conn = self.pool.get().await?;
		conn.execute(STMT_ADD_NAMESPACED_OBJECT, &[&ns_object.object_type, &ns_object.object_name, &ns_object.namespace, &ns_object.cluster]).await?;
		Ok(())
	}

	async fn get_namespaced_object_id(&self, ns_object: objects::NamespacedObject) -> Result<i32, Error> {
		let conn = self.pool.get().await?;
		let row = conn.query_one(STMT_GET_NAMESPACED_OBJECT_ID, &[&ns_object.object_type, &ns_object.object_name, &ns_object.namespace, &ns_object.cluster]).await?;
		let id: i32 = row.get(0);
		Ok(id)
	}

	async fn add_object_issue(&self, object_issue: objects::ObjectIssue) -> Result<(), Error> {
		let conn = self.pool.get().await?;
		conn.execute(STMT_ADD_OBJECT_ISSUE, &[
			&object_issue.object_id,
			&object_issue.category,
			&object_issue.details,
			&object_issue.severity,
			&object_issue.issue_tech_id,
			&object_issue.issue_message,
			&object_issue.reported_by,
			&object_issue.reported_at,
			&object_issue.last_seen_at,
			&object_issue.linked_object_id]).await?;
		Ok(())
	}

	async fn get_object_issues(&self, object_type: &str, object_name: &str, namespace: &str, cluster: &str) -> Result<Vec<objects::ObjectIssue>, Error> {
		let conn = self.pool.get().await?;
		let rows = conn.query(STMT_GET_OBJECT_ISSUES, &[&object_type, &object_name, &namespace, &cluster]).await?;
		let mut issues: Vec<objects::ObjectIssue> = Vec::new();
		for row in rows {
			let issue = objects::ObjectIssue {
				object_id: row.get(0),
				category: row.get(1),
				details: row.get(2),
				severity: row.get(3),
				issue_tech_id: row.get(4),
				issue_message: row.get(5),
				reported_by: row.get(6),
				reported_at: row.get(7),
				last_seen_at: row.get(8),
				linked_object_id: row.get(9),
			};
			issues.push(issue);
		}
		Ok(issues)
	}

}

// For database migrations
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

