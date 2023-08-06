use hyper::Error;
use axum::{
    routing,
    Router,
};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
// use utoipa_swagger_ui::SwaggerUi;
use std::net::SocketAddr;

#[derive(OpenApi)]
#[openapi(
	paths(
		namespaces::list,

		applications::list_gitops_applications,

		compute::list,

		issues::list_security_issues,
		issues::list_configuration_issues,
	),
	components(
		schemas(
			billing::BillingEntry,
			billing::BillingResult,
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
		//.merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
		.merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
		.route("/v1/namespaces", routing::get(namespaces::list))
		.route("/v1/applications/gitops/:namespace", routing::get(applications::list_gitops_applications))
		.route("/v1/compute/:namespace", routing::get(compute::list))
		.route("/v1/issues/security/:namespace", routing::get(issues::list_security_issues))
		.route("/v1/issues/configuration/:namespace", routing::get(issues::list_configuration_issues))
        .route("/v1/billing", routing::post(billing::add_entries))
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

mod applications {
	#[utoipa::path(
        get,
        path = "/v1/applications/gitops/:namespace",
        responses(
            (status = 200, description = "List all gitops applications successfully", body = [BillingResult])
        )
    )]
	pub(super) async fn list_gitops_applications() -> &'static str {
		"List Applications"
	}
}

mod namespaces {
	use axum::Json;

	#[utoipa::path(
        get,
        path = "/v1/namespaces",
        responses(
            (status = 200, description = "List all namespaces")
        )
    )]
	#[axum::debug_handler]
	pub(super) async fn list() -> Json<Vec<String>> {
		let r = vec!["default".to_string(), "kube-system".to_string()];
		Json(r)
	}
}

mod compute {
	#[utoipa::path(
        get,
        path = "/v1/compute/:namespace",
        responses(
            (status = 200, description = "List all compute successfully")
        )
    )]
	pub(super) async fn list() -> &'static str {
		"List Compute"
	}
}

mod issues {
	use axum::{
		http::StatusCode,
	};

	#[utoipa::path(
        get,
        path = "/v1/issues/security/:namespace",
        responses(
            (status = 200, description = "List all issues successfully")
        )
    )]
	pub(super) async fn list_security_issues() -> (StatusCode, &'static str) {
		let r = "List Security Issues";
		(StatusCode::OK, r)
	}

	#[utoipa::path(
        get,
        path = "/v1/issues/configuration/:namespace",
        responses(
            (status = 200, description = "List all configuration issues successfully", body = [BillingResult])
        )
    )]
	pub(super) async fn list_configuration_issues() -> (StatusCode, &'static str) {
		let r = "List Configuration Issues";
		(StatusCode::OK, r)
	}
}

mod billing {
	use axum::{
		http::StatusCode,
    };
	use utoipa::ToSchema;
	use serde::{Deserialize, Serialize};

	#[derive(Serialize, Deserialize, ToSchema, Clone)]
    pub(super) struct BillingEntry {
        id: i32,
        #[schema(example = "Buy groceries")]
        value: String,
        done: bool,
    }

	#[derive(Serialize, Deserialize, ToSchema, Clone)]
    pub(super) struct BillingResult {
        status: String,
    }

	#[utoipa::path(
        post,
        path = "/v1/billing",
        responses(
            (status = 200, description = "Publish billing states", body = [BillingResult])
        )
    )]
	pub(super) async fn add_entries() -> (StatusCode, &'static str) {
		let r ="OK";
		(StatusCode::CREATED, r)
	}
}
