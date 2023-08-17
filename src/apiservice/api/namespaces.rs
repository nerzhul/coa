use axum::{Extension, Json};
use hyper::StatusCode;
use k8s_openapi::api::core::v1::Namespace;
use kube::api::ListParams;
use log::error;

use super::helpers;

#[utoipa::path(
	get,
	path = "/v1/namespaces",
	responses(
		(status = 200, description = "List all namespaces")
	)
)]
pub async fn list(
    Extension(kube_client): Extension<kube::Client>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let mut result = vec![];

    let (username, groups) = helpers::get_user_context();

    let namespaces: kube::Api<Namespace> = kube::Api::all(kube_client.clone());
    match namespaces.list(&ListParams::default()).await {
        Ok(r) => {
            for ns in r.items {
                // Check individual namespace rights
                let namespace_name = match ns.metadata.name {
                    Some(n) => n,
                    None => {
                        error!("Namespace without name found");
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };

                match helpers::has_rights(&kube_client, &namespace_name, &username, &groups).await {
                    Ok(r) => {
                        if !r {
                            continue;
                        }
                    }
                    Err(e) => {
                        error!("Error while checking rights: {}", e);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }

                result.push(namespace_name);
            }
        }
        Err(e) => {
            error!("Error while listing namespaces: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Json(result))
}
