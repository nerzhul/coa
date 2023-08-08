use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use std::option::Option;

const STMT_GET_PRICING: &str = "SELECT price, period, description FROM pricing WHERE id = $1";
const STMT_GET_PRICING_ID_BY_OBJECT_TYPE: &str = "SELECT id FROM pricing WHERE object_type = $1";
const STMT_ADD_INVOICE: &str = "INSERT INTO invoice(object_type, object_name, start_time, end_time, price_id VALUES ($1, $2, $3, $4, $5)";
const STMT_GET_INVOICE_ID_BY_END_TIME: &str = "SELECT id FROM invoice WHERE object_type = $1 AND object_name = $2 AND end_time >= $3";

pub async fn create_pool(db_host: Option<String>, db_name: Option<String>, username: Option<String>, password: Option<String>, pool_size: usize) -> deadpool_postgres::Pool {
	let mut cfg = tokio_postgres::Config::new();
	match db_host {
		Some(h) => { cfg.host(h.as_str()); },
		None => {},
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


	let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Verified
    };
    let mgr = Manager::from_config(cfg, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(pool_size).build().unwrap();

	// let mut client = pool.get().await.unwrap();
	// let stmt = client.prepare_cached("SELECT 1");
	// let ping = client.query(&stmt, []).await.unwrap();
	// TODO: implement ping call
	pool
}