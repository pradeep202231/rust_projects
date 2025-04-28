use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use dotenv::dotenv;


pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    //Configure connection pool 
    Pool::builder()
    .max_size(5)
     .build(manager)
    .expect("Failed to create database pool")
}

pub fn get_conn(pool: &PgPool) ->PgPooledConnection{
    pool.get()
    .expect("Failed to get database connection from pool")
}