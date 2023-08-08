use axum::extract::Path;

#[utoipa::path(
	get,
	path = "/v1/applications/gitops/:namespace",
	responses(
		(status = 200, description = "List all gitops applications successfully", body = [BillingResult])
	)
)]
pub async fn list_gitops_applications(Path(namespace): Path<String>) -> &'static str {
	"List GitOps Applications"
}
