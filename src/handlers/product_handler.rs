use axum::{Extension, Json};
use sqlx::{prelude::FromRow, PgPool};

use crate::http::result::AppResult;

#[derive(FromRow, Debug, serde::Serialize)]
pub struct Product {
    id: i32, 
    name: String
}

pub async fn get_products(
    Extension(pool): Extension<PgPool>,
) -> AppResult<Json<Vec<Product>>> {
    let data = sqlx::query_as::<_, Product>("select id, name from products")
        .fetch_all(&pool)
        .await?;

    Ok(Json(data))
}