use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use std::option::Option;

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