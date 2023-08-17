use axum::extract::Path;

use crate::api::helpers;

#[utoipa::path(
	get,
	path = "/v1/applications/gitops/:namespace",
	responses(
		(status = 200, description = "List all gitops applications successfully", body = [BillingResult])
	)
)]
pub async fn list_gitops_applications(Path(namespace): Path<String>) -> String {
    let (username, groups) = helpers::get_user_context();

    format!("List GitOps Applications {}", namespace)
}
