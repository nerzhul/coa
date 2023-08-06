use axum::{
	http::StatusCode,
};
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct BillingEntry {
	id: i32,
	#[schema(example = "Buy groceries")]
	value: String,
	done: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct BillingResult {
	status: String,
}

#[utoipa::path(
	post,
	path = "/v1/billing",
	responses(
		(status = 200, description = "Publish billing states", body = [BillingResult])
	)
)]
pub async fn add_entries() -> (StatusCode, &'static str) {
	let r ="OK";
	(StatusCode::CREATED, r)
}