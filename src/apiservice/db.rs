use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use std::{
	option::Option,
	result::Result
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

const STMT_GET_PRICING: &str = "SELECT price, period, description FROM pricing WHERE id = $1";
const STMT_GET_PRICING_ID_BY_OBJECT_TYPE: &str = "SELECT id FROM pricing WHERE object_type = $1";
const STMT_ADD_INVOICE: &str = "INSERT INTO invoice(object_type, object_name, start_time, end_time, price_id VALUES ($1, $2, $3, $4, $5)";
const STMT_GET_INVOICE_ID_BY_END_TIME: &str = "SELECT id FROM invoice WHERE object_type = $1 AND object_name = $2 AND end_time >= $3";

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
}

// For database migrations
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

