use axum::{http::StatusCode, Json, extract::Path};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use super::helpers;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct IssueObject {
	id: String,
	cluster: String,
	namespace: String,
	object_type: String,
	object_name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Issue {
	#[schema(read_only = true)]
	id: Option<String>,
	#[schema(write_only = true)]
	cluster: String,
	#[schema(write_only = true)]
	namespace: String,
	#[schema(write_only = true)]
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
pub struct IssueList {
	pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct IssueListWithObjects {
	pub issues: Vec<Issue>,
	pub objects: Vec<IssueObject>,
}

#[derive(Deserialize, ToSchema)]
pub struct IssuesNamespaceParams {
	issue_type: String,
    namespace_name: String,
}

#[utoipa::path(
	get,
	path = "/v1/issues/:issue_type/:namespace",
	responses(
		(status = 200, description = "List issues successfully", body=[IssueListWithObjects])
	)
)]
pub async fn list_issues_by_type(Path(IssuesNamespaceParams{ issue_type, namespace_name }): Path<IssuesNamespaceParams>) -> (StatusCode, Json<IssueListWithObjects>) {
	let (username, groups) = helpers::get_user_context();

	let r: Json<IssueListWithObjects> = Json(IssueListWithObjects {
		issues: vec!(),
		objects: vec!(),
	});
	(StatusCode::OK, r)
}

#[utoipa::path(
	post,
	path = "/v1/issues",
	request_body = IssueList,
	responses(
		(status = 200, description = "Publish issues successfully"),
		(status = 500, description = "Server error")
	)
)]
pub async fn store_issues(Json(issue_list): Json<IssueList>) -> (StatusCode, &'static str) {
	let (username, groups) = helpers::get_user_context();

	let r = "Store Issues";
	(StatusCode::OK, r)
}