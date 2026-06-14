mod errors;
mod handlers;
mod models;
mod repository;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::repository::FriendRepository;

#[tokio::main]
async fn main() {
    let state = FriendRepository::new();

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
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
