use axum::{
    routing,
    Router,
};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;
use std::env;

mod api;
mod db;

#[derive(OpenApi)]
#[openapi(
	paths(
		api::namespaces::list,

		api::applications::list_gitops_applications,

		api::compute::list,

		api::issues::list_issues_by_type,
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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

	if env::var("KUBECONFIG").is_err() {
		eprintln!("KUBECONFIG environment variable not set");
		std::process::exit(1);
	}

	let db = db::Database::new(db_host, db_name, db_user, db_password, db_pool_size).await?;

	let ctx = api::ApiContext {
		db: db.clone(),
		kube_client: kube::Client::try_default().await.unwrap(),
	};

	// build our application with a route
    let app: Router<()> = Router::new()
        .route("/", routing::get(root))
		//.route("/openapi.json", routing::get(ApiDoc::openapi()))
		.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
		.merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
		.route("/v1/namespaces", routing::get(api::namespaces::list))
		.route("/v1/applications/gitops/:namespace", routing::get(api::applications::list_gitops_applications))
		.route("/v1/compute/:namespace", routing::get(api::compute::list))
		.route("/v1/issues/:issue_type/:namespace", routing::get(api::issues::list_issues_by_type))
        .route("/v1/billing/pod", routing::post(api::billing::post_pod_invoice))
		.route("/v1/health/liveness", routing::get(health::liveness))
		.route("/v1/health/readiness", routing::get(health::readiness))
		.with_state(ctx);

    let _ = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await;

	Ok(())
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




