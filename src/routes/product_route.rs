use axum::{routing::get, Extension, Router};
use sqlx::PgPool;
use crate::handlers::product_handler::get_products;

pub fn product_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/products", get(get_products))
        .layer(Extension(pool))
}
