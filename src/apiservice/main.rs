use hyper::Error;
use axum::{
    routing,
    Router,
};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;
use std::env;

mod api;
mod db;

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
			api::billing::PodBillingEntry,
			api::billing::BillingResult,
		),
	)
)]
struct ApiDoc;

fn result_to_option(result: Result<String, env::VarError>) -> Option<String> {
	result.ok()
 }

#[tokio::main]
async fn main() -> Result<(), Error> {
	let db_user = result_to_option(env::var("DB_USERNAME"));
	let db_password = result_to_option(env::var("DB_PASSWORD"));
	let db_name = result_to_option(env::var("DB_NAME"));
	let db_host = result_to_option(env::var("DB_HOST"));
	let db_pool_size: usize = match env::var("DB_POOL_SIZE") {
		Ok(pool_size) => match pool_size.parse::<usize>() {
			Ok(i) => i,
			Err(e) => {
				eprintln!("Failed to parse DB_POOL_SIZE: {}, not an integer: {}", pool_size, e);
				10
			}
		},
		Err(_e) => 10
	};

	let mut _db = db::create_pool(db_host, db_name, db_user, db_password, db_pool_size);

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
        .route("/v1/billing/pod", routing::post(api::billing::post_pod_invoice))
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




