#![recursion_limit = "256"]
#[macro_use] extern crate rocket;
mod model;
mod schema;
mod db;
mod error;
mod routes;

#[get("/greet")]
async fn hello()->&'static str{
    "pradeep"
}
#[rocket::main]
async fn main() {
    let _=rocket::build()
    .mount("/",rocket::routes![hello,
    routes::create_user,
    routes::create_post,
    routes::list_posts])
    .manage(db::establish_connection())
    .launch()
    .await;
}
