use axum::{extract::Path, http::StatusCode, Extension, Json};
use log::error;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::db::Database;

use super::helpers;
use super::objects::NamespacedObject;

#[derive(Debug, FromSql, ToSql, Deserialize, Serialize, Clone, ToSchema)]
#[postgres(name = "issue_category", rename_all = "lowercase")]
pub enum IssueCategory {
    Security,
    Reliability,
    Performance,
    Configuration,
    Unknown,
}

#[derive(Debug, FromSql, ToSql, Deserialize, Serialize, Clone, ToSchema)]
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
    pub object_id: Uuid,
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
pub struct ObjectWithIssues {
    pub metadata: NamespacedObject,
    pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct IssueListWithObjects {
    pub issues: Vec<ObjectWithIssues>,
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

    let mut r = IssueListWithObjects { issues: vec![] };

    r.issues = match db
        .get_objects_with_issue_category_in_namespace(category.clone(), &namespace_name)
        .await
    {
        Ok(objects) => {
            let issues = vec![];
            for object in objects {
                r.issues.push(ObjectWithIssues {
                    metadata: object.clone(),
                    issues: vec![],
                });
            }
            issues
        }
        Err(e) => {
            eprintln!(
                "Unable to run db.get_objects_with_issue_category_in_namespace : {}",
                e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match db
        .get_issues_with_category_for_namespace(category, &namespace_name)
        .await
    {
        Ok(issues) => {
            for issue in issues {
                for object in r.issues.iter_mut() {
                    if object.metadata.id == issue.object_id {
                        object.issues.push(issue);
                        break;
                    }
                }
            }
        }
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
pub async fn store_issues(
    Extension(db): Extension<Database>,
    Json(issue_list): Json<IssueList>,
) -> (StatusCode, &'static str) {
    // TODO: check rights, it require another authent than the kube one

	for issue in issue_list.issues {
		let id = match db.record_namespaced_object(&issue.object_type, &issue.object_name, &issue.cluster, &issue.namespace).await {
			Ok(id) => {
				id
			}
			Err(e) => {
				eprintln!("Unable to run db.record_namespaced_object : {}", e);
				return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to run db.record_namespaced_object");
			}
		};

		let issue = Issue {
			object_id: id,
			category: issue.category,
			details: issue.details.unwrap_or("".to_string()),
			severity: issue.severity,
			issue_tech_id: issue.issue_tech_id,
			issue_message: issue.issue_message,
			reported_by: issue.reported_by.unwrap_or("".to_string()),
			reported_at: issue.reported_at.unwrap_or("".to_string()),
			last_seen_at: issue.last_seen_at.unwrap_or("".to_string()),
			linked_object_id: issue.linked_object_id.unwrap_or("".to_string()),
		};

		match db.add_object_issue(issue).await {
			Ok(_) => {}
			Err(e) => {
				eprintln!("Unable to run add_object_issue.add_issue : {}", e);
				return (StatusCode::INTERNAL_SERVER_ERROR, "Unable to run db.add_issue");
			}
		}
	}

    (StatusCode::OK, "{}")
}
