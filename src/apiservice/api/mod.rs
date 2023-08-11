pub mod applications;
pub mod billing;
pub mod compute;
pub mod issues;
pub mod objects;
pub mod namespaces;
mod kube_helpers;

use kube::Client;
use crate::db;

#[derive(Clone)]
pub struct ApiContext {
	pub db: db::Database,
	pub kube_client: Client,
}