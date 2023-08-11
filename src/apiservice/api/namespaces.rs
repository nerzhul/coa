use axum::{Json, extract::State};
use hyper::StatusCode;
use kube::{
	Api as KubeApi,
	api::ListParams,
	Client,
};
use k8s_openapi::api::core::v1::Namespace;
use crate::api::ApiContext;

#[utoipa::path(
	get,
	path = "/v1/namespaces",
	responses(
		(status = 200, description = "List all namespaces")
	)
)]
pub async fn list(State(ctx): State<ApiContext>) -> Result<Json<Vec<String>>, StatusCode> {
	let mut r = vec![];

	// TODO: implement auth & filtering based on rights/rbac

	let namespaces: KubeApi<Namespace> = KubeApi::all(ctx.kube_client);
	namespaces.list(&ListParams::default()).await.unwrap().items.into_iter().for_each(|ns| {
		r.push(ns.metadata.name.unwrap());
	});

	Ok(Json(r))
}