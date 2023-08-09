use axum::{
	http::StatusCode,
};

#[utoipa::path(
	get,
	path = "/v1/issues/:issue_type/:namespace",
	responses(
		(status = 200, description = "List issues successfully")
	)
)]
pub async fn list_issues_by_type() -> (StatusCode, &'static str) {
	let r = "List Security Issues";
	(StatusCode::OK, r)
}
