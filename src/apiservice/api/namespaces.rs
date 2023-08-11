use axum::Json;
use kube::{
	Api as KubeApi,
	api::ListParams,
	Client,
};
use k8s_openapi::api::core::v1::Namespace;

#[utoipa::path(
	get,
	path = "/v1/namespaces",
	responses(
		(status = 200, description = "List all namespaces")
	)
)]
pub async fn list() -> Json<Vec<String>> {
	// TODO: implement auth & filtering based on rights/rbac
	let mut r = vec![];

	let client = Client::try_default().await.unwrap();
	let namespaces: KubeApi<Namespace> = KubeApi::all(client);
	namespaces.list(&ListParams::default()).await.unwrap().items.into_iter().for_each(|ns| {
		r.push(ns.metadata.name.unwrap());
	});

	Json(r)
}