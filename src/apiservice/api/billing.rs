use axum::{
	http::StatusCode,
	Json,
};
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

use super::helpers;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct PodBillingEntry {
	namespace: String,
	pod_name: String,
	start_time: Option<String>,
	end_time: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct BillingResult {
	status: String,
}

#[utoipa::path(
	post,
	path = "/v1/billing/pod",
	responses(
		(status = 200, description = "Publish billing states", body = [BillingResult])
	)
)]
pub async fn post_pod_invoice() -> (StatusCode, Json<BillingResult>) {
	let (username, groups) = helpers::get_user_context();

	let r = BillingResult{status: "OK".to_string()};
	(StatusCode::CREATED, Json(r.clone()))
}