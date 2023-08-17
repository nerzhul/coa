use std::error::Error;
use std::fmt::{self, Display, Formatter};

use axum::{extract::Path, http::StatusCode, Extension, Json};
use log::error;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, Type};
use utoipa::ToSchema;

use crate::db::Database;

use super::helpers;
use super::objects::NamespacedObject;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub enum IssueCategory {
    Security,
    Reliability,
    Performance,
    Configuration,
    Unknown,
}

impl FromSql<'_> for IssueCategory {
    fn from_sql(_sql_type: &Type, value: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match value {
            b"security" => Ok(IssueCategory::Security),
            b"reliability" => Ok(IssueCategory::Reliability),
            b"performance" => Ok(IssueCategory::Performance),
            b"configuration" => Ok(IssueCategory::Configuration),
            _ => Ok(IssueCategory::Unknown),
        }
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type.name() == "issue_category"
    }
}

pub trait ToSql: fmt::Debug {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized;

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized;

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>>;
}

impl Display for IssueCategory {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            IssueCategory::Security => write!(f, "security"),
            IssueCategory::Reliability => write!(f, "reliability"),
            IssueCategory::Performance => write!(f, "performance"),
            IssueCategory::Configuration => write!(f, "configuration"),
            IssueCategory::Unknown => write!(f, "unknown"),
        }
    }
}

impl ToSql for IssueCategory {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        format!("{}", self).to_sql(ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type.name() == "myenum"
    }

    to_sql_checked!();
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Issue {
    pub object_id: String,
    pub category: IssueCategory,
    pub details: String,
    pub severity: String,
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
    category: String,
    details: Option<String>,
    severity: String,
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
    issue_type: String,
    namespace_name: String,
}

#[utoipa::path(
	get,
	path = "/v1/issues/{issue_type}/{namespace}",
	responses(
		(status = 200, description = "List issues successfully", body=IssueListWithObjects),
		(status = 500, description = "Server error")
	),
	params(
		("issue_type", Path, description = "Issue type"),
		("namespace", Path, description = "Namespace name")
	)
)]
pub async fn list_issues_by_type(
    Extension(db): Extension<Database>,
    Extension(kube_client): Extension<kube::Client>,
    Path(IssuesNamespaceParams {
        issue_type,
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
        .get_objects_with_issue_category_in_namespace(&issue_type, &namespace_name)
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
        .get_issues_with_category_for_namespace(&issue_type, &namespace_name)
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
