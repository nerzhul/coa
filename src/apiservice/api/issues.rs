use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use super::helpers;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Issue {
	#[schema(read_only = true)]
	id: Option<String>,
	cluster: String,
	namespace: String,
	object_name: String,
	#[schema(example = "Deployment")]
	object_type: String,
	category: String,
	details: Option<String>,
	severity: String,
	issue_tech_id: String,
	issue_message: String,
	reported_by: Option<String>,
	reported_at: Option<String>,
	#[schema(read_only = true)]
	last_seen_at: Option<String>,
	linked_object_id: Option<String>
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct IssueBulkList {
	pub issues: Vec<Issue>,
}

#[utoipa::path(
	get,
	path = "/v1/issues/:issue_type/:namespace",
	responses(
		(status = 200, description = "List issues successfully")
	)
)]
pub async fn list_issues_by_type() -> (StatusCode, &'static str) {
	let (username, groups) = helpers::get_user_context();

	let r = "List Issues";
	(StatusCode::OK, r)
}

#[utoipa::path(
	post,
	path = "/v1/issues",
	request_body = IssueBulkList,
	responses(
		(status = 200, description = "Publish issues successfully"),
		(status = 500, description = "Server error")
	)
)]
pub async fn store_issues(Json(issue_list): Json<IssueBulkList>) -> (StatusCode, &'static str) {
	let (username, groups) = helpers::get_user_context();

	let r = "Store Issues";
	(StatusCode::OK, r)
}