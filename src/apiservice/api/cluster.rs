use axum::{Json, Extension};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq)]
pub struct ClusterIdentity {
	name: String,
}

impl ClusterIdentity {
	pub fn new(name: String) -> Self {
		Self { name }
	}
}

#[utoipa::path(
	get,
	path = "/v1/cluster",
	responses(
		(status = 200, description = "Get current cluster info", body = ClusterIdentity)
	)
)]
pub async fn get(Extension(cluster_identity): Extension<ClusterIdentity>) -> Json<ClusterIdentity> {
	Json(cluster_identity)
}
