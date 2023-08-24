use axum::{error_handling::HandleErrorLayer, routing, BoxError, Extension, Router};
use hyper::StatusCode;
use log::{error, info};
use std::env;
use tower::timeout::TimeoutLayer;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod db;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::cluster::get,
        api::namespaces::list,
        api::applications::list_gitops_applications,
        api::compute::list,
        api::billing::post_pod_invoice,
		
        api::issues::list_issues_by_category,
        api::issues::store_issues,
    ),
    components(schemas(
        api::objects::NamespacedObject,
        api::billing::PodBillingEntry,
        api::billing::BillingResult,
        api::cluster::ClusterIdentity,

		api::issues::ObjectWithIssues,
		api::issues::IssueCategory,
		api::issues::IssueSeverity,
        api::issues::IssuesNamespaceParams,
        api::issues::Issue,
        api::issues::PostIssue,
        api::issues::IssueList,
        api::issues::IssueListWithObjects,
    ),)
)]
struct ApiDoc;

fn result_to_option(result: Result<String, env::VarError>) -> Option<String> {
    result.ok()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    std_logger::Config::logfmt().init();

    let db_user = result_to_option(env::var("DB_USERNAME"));
    let db_password = result_to_option(env::var("DB_PASSWORD"));
    let db_name = result_to_option(env::var("DB_NAME"));
    let db_host = result_to_option(env::var("DB_HOST"));
    let db_pool_size: usize = match env::var("DB_POOL_SIZE") {
        Ok(pool_size) => match pool_size.parse::<usize>() {
            Ok(i) => i,
            Err(e) => {
                eprintln!(
                    "Failed to parse DB_POOL_SIZE: {}, not an integer: {}",
                    pool_size, e
                );
                10
            }
        },
        Err(_e) => 10,
    };

    let cluster_name: String = match env::var("CLUSTER_NAME") {
        Ok(cluster_name) => cluster_name,
        Err(_e) => "unknown".to_string(),
    };

    let request_timeout = match env::var("REQUEST_TIMEOUT") {
        Ok(timeout) => match timeout.parse::<u64>() {
            Ok(i) => i,
            Err(e) => {
                eprintln!(
                    "Failed to parse REQUEST_TIMEOUT: {}, not an integer: {}",
                    timeout, e
                );
                60000
            }
        },
        Err(_e) => 60000,
    };

    info!("request timeout set to {}ms", request_timeout);

    if env::var("KUBECONFIG").is_err() {
        eprintln!("KUBECONFIG environment variable not set");
        std::process::exit(1);
    }

    let db = db::Database::new(db_host, db_name, db_user, db_password, db_pool_size).await?;
    let kube_client = kube::Client::try_default().await.unwrap();

    // build our application with a route
    let app: Router<()> = Router::new()
        .route("/", routing::get(root))
        //.route("/openapi.json", routing::get(ApiDoc::openapi()))
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/v1/cluster", routing::get(api::cluster::get))
        .route("/v1/namespaces", routing::get(api::namespaces::list))
        .route(
            "/v1/applications/gitops/:namespace_name",
            routing::get(api::applications::list_gitops_applications),
        )
        .route("/v1/compute/:namespace", routing::get(api::compute::list))
        .route(
            "/v1/issues/:category/:namespace_name",
            routing::get(api::issues::list_issues_by_category),
        )
        .route("/v1/issues", routing::post(api::issues::store_issues))
        .route(
            "/v1/billing/pod",
            routing::post(api::billing::post_pod_invoice),
        )
        .route("/v1/health/liveness", routing::get(health::liveness))
        .route("/v1/health/readiness", routing::get(health::readiness))
        .layer(
            tower::ServiceBuilder::new()
                .layer(Extension(db))
                .layer(Extension(kube_client))
                .layer(Extension(api::cluster::ClusterIdentity::new(cluster_name)))
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    error!("request timeout");
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(std::time::Duration::from_millis(
                    request_timeout,
                ))),
        );

    match axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }

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
