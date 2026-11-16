#![allow(unused)]
use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use axum::response::{Html, IntoResponse};
use axum::extract::{Query};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(|| async {
        Html("Hello <strong>World!!!</strong>")
    }));

    // region: -- start server
    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("--> Listening on {addr} \n");
    axum::Server::bind(&addr).serve(routes_hello.into_make_service()).await.unwrap();
    // endregion: -- start server
    println!("Hello, world!");
}

#[derive(Debug,Deserialize)]
struct  HelloParams {
    name: Option<String>,
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    // Ok(Html("Hello <strong>World!!!</strong>"))
    println!("->> {:<12} - hello_handler - {params:?}","HANDLER");

    let name = params.name.as_deref().unwrap();

    Html("Hello <strong>World!!!</strong>")
}