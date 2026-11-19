#![allow(unused)]

use crate::context::Ctx;
pub use crate::error::{Error,Result};

use crate::log::log_request;
use crate::model::{ModelController};
use axum::{Json, Router, middleware};
use axum::routing::{get, get_service};
use axum::http::{Method,Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::extract::{Path, Query};
use serde_json::json;
use tower_cookies::{CookieManagerLayer};
use uuid::Uuid;
use std::net::SocketAddr;

use serde::Deserialize;
use tower_http::services::ServeDir;

mod context;
mod error;
mod log;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let route_apis = web::ticket_routes::ticket_routes(mc.clone())
    .route_layer(middleware::from_fn(web::middleware_auth::require_auth_middleware));

    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::login_routes::routes())
    // .merge(web::ticket_routes::routes(mc.clone()))
    .nest("/api", route_apis)
    .layer(middleware::map_response(main_response_mapper))
    .layer(middleware::from_fn_with_state(mc.clone(), web::middleware_auth::ctx_resolver_middleware))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    // region: -- start server
    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("--> Listening on {addr} \n");
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await;
    // endregion: -- start server
    println!("Hello, world!");

    Ok(())
}

async fn main_response_mapper(	ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,res:Response) -> Response {
    println!("->> {:<12} - main_response_mapper","HANDLER");
    let uuid = Uuid::new_v4();
    // get eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map( |se| se.client_error_and_status());

    // if client error
    let error_response = client_status_error.as_ref().map(|(sc,ce)| {
        let ce_body = json!({
            "error": {
                "type": ce.as_ref(),
                "req_uuid": uuid.to_string()
            }
        });

        println!("  ->> client_error_body: {ce_body:?}");
        (*sc,Json(ce_body)).into_response()
    });

    // build and log the server log line
    println!("  ->> server log line - {uuid} - Error: {service_error:?}");
    	// foo: Option[(String, Int)]; foo.map(_._2)
	let client_error = client_status_error.unzip().1;
	log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
    println!();
    error_response.unwrap_or(res)
}


// region -> Routes Static

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}


// region: -> Routes hello
fn routes_hello() -> Router {
    let routes = Router::new()
    .route("/hello", get(hello_handler))
    .route("/hello2/:name", get(hello2_handler));

    routes
}

#[derive(Debug,Deserialize)]
struct  HelloParams {
    name: Option<String>,
}


async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    // Ok(Html("Hello <strong>World!!!</strong>"))
    println!("->> {:<12} - hello_handler - {params:?}","HANDLER");

    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong>{name}</strong>"))
}


//. /hello/:name -> /hello/mike
async fn hello2_handler(Path(name):Path<String>) -> impl  IntoResponse {
    println!("->> {:<12} - hello2_handler - {name:?}","HANDLER");

    let name = name.trim().to_owned();

    Html(format!("Hello2 <strong>{name}</strong>"))
}