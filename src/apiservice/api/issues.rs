use axum::{
	http::StatusCode,
};

#[utoipa::path(
	get,
	path = "/v1/issues/security/:namespace",
	responses(
		(status = 200, description = "List all issues successfully")
	)
)]
pub async fn list_security_issues() -> (StatusCode, &'static str) {
	let r = "List Security Issues";
	(StatusCode::OK, r)
}

#[utoipa::path(
	get,
	path = "/v1/issues/configuration/:namespace",
	responses(
		(status = 200, description = "List all configuration issues successfully", body = [BillingResult])
	)
)]
pub async fn list_configuration_issues() -> (StatusCode, &'static str) {
	let r = "List Configuration Issues";
	(StatusCode::OK, r)
}