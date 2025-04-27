#![allow(dead_code)]

mod http;
mod routes;
mod handlers;
mod db;

use http::result::AppResult;
use http::server::Server;
use std::net::{IpAddr, SocketAddr};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();

    let ip: IpAddr = "127.0.0.1".parse()?;
    let port = 3030;
    let addr = SocketAddr::new(ip, port);

    let database_url = env::var("DATABASE_URL")?;
    let pool = db::init_pool(&database_url).await;

    let server = Server::new(addr, pool);
    server.run().await?;

    Ok(())
}

