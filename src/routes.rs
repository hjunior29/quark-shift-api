use axum::{
    Router,
    http::{
        Method, Request, Response,
        header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue},
    },
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use std::{env, time::Duration};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::handlers::{home, ping, redirect, shorten};

pub fn create_routes(db: DatabaseConnection) -> Router {
    let origin = env::var("ORIGIN_URL").expect("ORIGIN_URL must be set");

    let cors = CorsLayer::new()
        .allow_origin(
            origin
                .parse::<HeaderValue>()
                .unwrap_or(HeaderValue::from_static("*")),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    let trace = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &tracing::Span| {
            info!("--> Request: {} {}", request.method(), request.uri());
        })
        .on_response(
            |response: &Response<_>, latency: Duration, _span: &tracing::Span| {
                info!(
                    "<-- Response: {} (elapsed={}ms)",
                    response.status(),
                    latency.as_millis(),
                );
            },
        );

    Router::new()
        .route("/", get(home))
        .route("/shorten", post(shorten))
        .route("/{code}", get(redirect))
        .route("/ping", get(ping))
        .with_state(db)
        .layer(cors)
        .layer(trace)
}
