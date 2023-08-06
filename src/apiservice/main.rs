use hyper::Error;
use axum::{
    routing,
    Router,
};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
mod api;

#[derive(OpenApi)]
#[openapi(
	paths(
		api::namespaces::list,

		api::applications::list_gitops_applications,

		api::compute::list,

		api::issues::list_security_issues,
		api::issues::list_configuration_issues,
	),
	components(
		schemas(
			api::billing::BillingEntry,
			api::billing::BillingResult,
		),
	)
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Error> {



    // build our application with a route
    let app = Router::new()
        .route("/", routing::get(root))
		//.route("/openapi.json", routing::get(ApiDoc::openapi()))
		.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
		.merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
		.route("/v1/namespaces", routing::get(api::namespaces::list))
		.route("/v1/applications/gitops/:namespace", routing::get(api::applications::list_gitops_applications))
		.route("/v1/compute/:namespace", routing::get(api::compute::list))
		.route("/v1/issues/security/:namespace", routing::get(api::issues::list_security_issues))
		.route("/v1/issues/configuration/:namespace", routing::get(api::issues::list_configuration_issues))
        .route("/v1/billing", routing::post(api::billing::add_entries))
		.route("/v1/health/liveness", routing::get(health::liveness))
		.route("/v1/health/readiness", routing::get(health::readiness));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Coa API Service"
}

mod health {
	#[utoipa::path(
		get,
		path = "/v1/health/liveness",
		responses(
			(status = 200, description = "Liveness probe")
		)
	)]
	pub(super) async fn liveness() -> &'static str {
		"OK"
	}

	#[utoipa::path(
		get,
		path = "/v1/health/readiness",
		responses(
			(status = 200, description = "Readiness probe")
		)
	)]
	pub(super) async fn readiness() -> &'static str {
		"OK"
	}
}




