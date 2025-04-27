use axum::{Router, routing::get};
use crate::handlers::user_handler::*;

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/users", get(get_users))
}
