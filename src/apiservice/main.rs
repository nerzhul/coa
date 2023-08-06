use axum::{
    routing,
    Router,
};
use utoipa::OpenApi;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
	#[derive(OpenApi)]
    #[openapi(
        paths(
            issues::list_security_issues,
            issues::list_configuration_issues,
			billing::add_entries
        ),
        components(
            schemas(billing::BillingEntry, billing::BillingResult)
        ),
        tags(
            (name = "billing", description = "billing API")
        )
    )]
	struct ApiDoc;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", routing::get(root))
		.route("/v1/issues/security", routing::get(issues::list_security_issues))
		.route("/v1/issues/configuration", routing::get(issues::list_configuration_issues))
        // `POST /v1/billing` register new billing entries
        .route("/v1/billing", routing::post(billing::add_entries));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Coa API Service"
}

mod issues {
	use axum::{
		http::StatusCode,
	};

	#[utoipa::path(
        get,
        path = "/v1/issues/security",
        responses(
            (status = 200, description = "List all todos successfully", body = [BillingResult])
        )
    )]
	pub(super) async fn list_security_issues() -> (StatusCode, &'static str) {
		let r = "List Security Issues";
		(StatusCode::OK, r)
	}

	#[utoipa::path(
        get,
        path = "/v1/issues/configuration",
        responses(
            (status = 200, description = "List all todos successfully", body = [BillingResult])
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
            (status = 200, description = "List all todos successfully", body = [BillingResult])
        )
    )]
	pub(super) async fn add_entries() -> (StatusCode, &'static str) {
		let r ="OK";
		(StatusCode::CREATED, r)
	}
}
