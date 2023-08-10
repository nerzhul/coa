use axum::{Json, extract::State};

use crate::db::Database;

#[utoipa::path(
	get,
	path = "/v1/namespaces",
	responses(
		(status = 200, description = "List all namespaces")
	)
)]
pub async fn list(State(db): State<Database>) -> Json<Vec<String>> {
	let r = vec!["default".to_string(), "kube-system".to_string()];
	Json(r)
}