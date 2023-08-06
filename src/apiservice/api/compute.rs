#[utoipa::path(
	get,
	path = "/v1/compute/:namespace",
	responses(
		(status = 200, description = "List all compute successfully")
	)
)]
pub async fn list() -> &'static str {
	"List Compute"
}