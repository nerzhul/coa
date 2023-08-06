use axum::Json;

#[utoipa::path(
	get,
	path = "/v1/namespaces",
	responses(
		(status = 200, description = "List all namespaces")
	)
)]
pub async fn list() -> Json<Vec<String>> {
	let r = vec!["default".to_string(), "kube-system".to_string()];
	Json(r)
}