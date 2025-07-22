use axum::{
    Router,
    routing::{get, post},
};
use http::{Request, Response};
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::{info};

use crate::handlers::{ping_handler, redirect_handler, shorten_handler};

pub fn create_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/shorten", post(shorten_handler))
        .route("/{code}", get(redirect_handler))
        .route("/ping", get(ping_handler))
        .with_state(db)
        .layer(
            TraceLayer::new_for_http()
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
                ),
        )
}
