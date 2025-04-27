use sqlx::PgPool;

use crate::http::result::AppResult;
use crate::routes;
use std::net::SocketAddr;

pub struct Server {
    addr: SocketAddr,
    pool: PgPool,
}

impl Server {
    pub fn new(addr: SocketAddr, pool: PgPool) -> Self {
        Self { addr, pool }
    }

    pub async fn run(&self) -> AppResult<()> {
        let app = routes::create_routes(self.pool.clone());

        println!("listening on {}", self.addr);

        axum_server::bind(self.addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}