mod api;
pub mod dtos;
pub mod extractors;
pub mod services;
pub mod utils;
pub mod error;

use std::future::ready;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::Context;
use axum::extract::MatchedPath;
use axum::http::{HeaderValue, Request};
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Extension;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Json, Router};
use lazy_static::lazy_static;
use serde_json::json;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{cors::Any, cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, info};

use crate::config::Config;
use crate::database::Database;
use crate::server::services::Services;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] =
        &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct Server;

impl Server {
    pub async fn serve(config: Arc<Config>, db: Database) -> anyhow::Result<()> {
        let services = Services::new(db, config.clone());

        let cors_origin = &config.cors_origin;

        let cors = CorsLayer::new()
            .allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers(Any);

        let router = Router::new()
            .route("/", get(api::health)) // TODO: Fix the route.
            .nest("/api/v1", api::app()) // TODO: Fix the route.
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT))
                    .layer(cors)
                    .layer(Extension(services))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(100, Duration::from_secs(1))),
            );

        let router = router.fallback(Self::handle_404);

        let port = config.port;
        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .context("Failed to initialize a TCP listener.")?;
            
        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("Failed to start the API server.")?;

        debug!("Server routes initialized, listening on port: {}.", port);
        info!("🚀 Server has launched on \'http://{addr}\'.");

        Ok(())
    }

    /// Adds a custom handler for tower's `TimeoutLayer`, see https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware.
    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error": "The request has timed out."
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Unhandled internal error: {}", err)
                })),
            )
        }
    }

    /// Tokio signal handler that will wait for a user to press CTRL+C.
    /// We use this in our hyper `Server` method `with_graceful_shutdown`.
    async fn shutdown_signal() {
        tokio::signal::ctrl_c()
            .await
            .expect("Expect the tokio signal \'Ctrl+c\'");
        info!("The shutdown signal has been detected.");
    }

    async fn handle_404() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            axum::response::Json(serde_json::json!({
            "errors":{
            "message": vec!(String::from("The requested resource does not exist on the server.")),}
            })),
        )
    }
}