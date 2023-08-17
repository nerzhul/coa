use super::helpers;

#[utoipa::path(
	get,
	path = "/v1/compute/:namespace",
	responses(
		(status = 200, description = "List all compute successfully")
	)
)]
pub async fn list() -> &'static str {
    let (username, groups) = helpers::get_user_context();

    "List Compute"
}
