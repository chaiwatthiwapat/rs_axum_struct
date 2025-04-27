pub mod user_route;
pub mod product_route;

use axum::{Router, routing::get, http::{Method, header, HeaderValue}, Extension};
use product_route::product_routes;
use tower_http::cors::CorsLayer;
use user_route::user_routes;
use sqlx::PgPool;

pub fn create_routes(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true);

    Router::new()
        .route("/", get(|| async { "Home" }))
        .route("/api/hello", get(|| async { "Hello" }))
        .merge(user_routes())
        .merge(product_routes(pool.clone()))
        .layer(cors)
        .layer(Extension(pool))
}