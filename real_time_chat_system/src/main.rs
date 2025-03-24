use api::auth::login;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use axum::{Router,routing::{get,post},Extension};
use axum::response::Response;
use std::net::SocketAddr;
use redis::Client as RedisClient;

mod api;
mod utils;
mod tests;

use crate::api::auth::register;

pub async fn hello()->& 'static str{
    "hello"
}
#[tokio::main]
async fn main(){
    dotenv().ok();

    let db_url=std::env::var("DATABASE_URL").unwrap();
    let pool=PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await
    .expect("Failed to connect to PostgresSQL");

    let redis_client=RedisClient::open(std::env::var("REDIS_URL").unwrap()).unwrap();

    let app= Router::new()
    .route("/ws/:room",get(api::chat::websocket_handler))
    .route("/register",post(register))
    .route("/login",post(login))
    .layer(Extension(pool))
    .layer(Extension(redis_client));

    let addr=SocketAddr::from(([127,0,0,1], 3000));
    println!("Server running on address {}",addr);
    axum::Server::bind(&addr)
         .serve(app.into_make_service())
         .await
         .unwrap();

   }