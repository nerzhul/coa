use axum::{extract::Path, http::StatusCode, Extension, Json};
use log::error;
use serde::{Deserialize, Serialize};
use postgres_types::{ToSql, FromSql};
use utoipa::ToSchema;

use crate::db::Database;

use super::helpers;
use super::objects::NamespacedObject;

#[derive(Debug, FromSql, ToSql, Deserialize, Serialize, Clone)]
#[postgres(name = "issue_category", rename_all = "lowercase")]
pub enum IssueCategory {
    Security,
    Reliability,
    Performance,
    Configuration,
    Unknown,
}

#[derive(Debug, FromSql, ToSql, Deserialize, Serialize, Clone)]
#[postgres(name = "issue_severity", rename_all = "lowercase")]
pub enum IssueSeverity {
	Critical,
	High,
	Medium,
	Low,
	Unknown,
}

#[derive(Serialize, Deserialize, FromSql, ToSql, ToSchema, Clone, Debug)]
pub struct Issue {
    pub object_id: String,
    pub category: IssueCategory,
    pub details: String,
    pub severity: IssueSeverity,
    pub issue_tech_id: String,
    pub issue_message: String,
    pub reported_by: String,
    pub reported_at: String,
    pub last_seen_at: String,
    pub linked_object_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct PostIssue {
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
    category: IssueCategory,
    details: Option<String>,
    severity: IssueSeverity,
    issue_tech_id: String,
    issue_message: String,
    reported_by: Option<String>,
    reported_at: Option<String>,
    #[schema(read_only = true)]
    last_seen_at: Option<String>,
    linked_object_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct IssueList {
    pub issues: Vec<PostIssue>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct IssueListWithObjects {
    pub issues: Vec<Issue>,
    pub objects: Vec<NamespacedObject>,
}

#[derive(Deserialize, ToSchema)]
pub struct IssuesNamespaceParams {
    category: IssueCategory,
    namespace_name: String,
}

#[utoipa::path(
	get,
	path = "/v1/issues/{category}/{namespace}",
	responses(
		(status = 200, description = "List issues successfully", body=IssueListWithObjects),
		(status = 500, description = "Server error")
	),
	params(
		("issue_type", Path, description = "Issue type"),
		("namespace", Path, description = "Namespace name")
	)
)]
pub async fn list_issues_by_category(
    Extension(db): Extension<Database>,
    Extension(kube_client): Extension<kube::Client>,
    Path(IssuesNamespaceParams {
        category,
        namespace_name,
    }): Path<IssuesNamespaceParams>,
) -> Result<Json<IssueListWithObjects>, StatusCode> {
    let (username, groups) = helpers::get_user_context();
    match helpers::has_rights(&kube_client, &namespace_name, &username, &groups).await {
        Ok(r) => {
            if !r {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        Err(e) => {
            error!("Error while checking rights: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let mut r = IssueListWithObjects {
        issues: vec![],
        objects: vec![],
    };

    r.objects = match db
        .get_objects_with_issue_category_in_namespace(category.clone(), &namespace_name)
        .await
    {
        Ok(objects) => objects,
        Err(e) => {
            eprintln!(
                "Unable to run db.get_objects_with_issue_category_in_namespace : {}",
                e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    r.issues = match db
        .get_issues_with_category_for_namespace(category, &namespace_name)
        .await
    {
        Ok(issues) => issues,
        Err(e) => {
            eprintln!(
                "Unable to run db.get_issues_with_category_for_namespace : {}",
                e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(r))
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
