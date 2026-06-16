mod entities;
mod errors;
mod handlers;
mod models;
mod repository;

use axum::{Router, routing::get};
use sea_orm::Database;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use crate::repository::FriendRepository;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is required");
    let connection = Database::connect(url)
        .await
        .expect("failed to connect to the database");
    let state = FriendRepository::new(connection);

    let app = Router::new()
        .route(
            "/friends",
            get(handlers::list_friends).post(handlers::create_friend),
        )
        .route(
            "/friends/{id}",
            get(handlers::get_friend)
                .delete(handlers::delete_friend)
                .put(handlers::update_friend),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::TRACE))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!(addr = %listener.local_addr().unwrap(), "listening");
    axum::serve(listener, app).await.unwrap();
}
