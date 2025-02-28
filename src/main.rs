#[macro_use] extern crate rocket;
mod migrator;
mod db;
mod controller;
mod entities;
mod auth;

use auth::cors::cors_options;
use controller::contact_controller;
use migrator::Migrator;
use sea_orm_migration::MigratorTrait;
use crate::controller::google_login::google_sign_in;
use crate::controller::{user_controller::{sign_up,sign_in},contact_controller::{get_all_contact,add_contact,update_contact,get_contact,delete_contact}};

use crate::auth::AuthenticatedUser;

pub struct AppConfig{
    db_host:String,
    db_port:String,
    db_username:String,
    db_password:String,
    db_database:String,
    jwt_secret: String,
}

impl Default for AppConfig{
    fn default()->Self{
        Self { db_host: std::env::var("DB_HOST").unwrap_or("localhost".to_string()),
         db_port: std::env::var("DB_PORT").unwrap_or("5432".to_string()),
         db_username: std::env::var("DB_USERNAME").unwrap_or("postgres".to_string()),
         db_password: std::env::var("DB_PASSWORD").unwrap_or("pradeep".to_string()),
          db_database: std::env::var("DB_DATABASE").unwrap_or("postgres".to_string()),
          jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET")
          .unwrap_or("rgbebgerghebrhgberjhhbgjehgjebgjebtjhgbtehjbgjthebgjebtdbgdjbgdtbgjtbgjb".to_string()),
         }
    }
}


#[get("/")]
async fn hello(user: AuthenticatedUser) -> String {
    format!("Authenticated user ID: {}", user.id)
}


#[launch]
async fn rocket()->_ {
    let config=AppConfig::default();

    let db=match db::connect(&config).await{
        Ok(db)=>db,
        Err(err)=>panic!("{}",err)
    };

   if let Err(err) = Migrator::up(&db,None).await { panic!("{}",err) }


    
    rocket::build()
    .attach(cors_options())
    .manage(db)
    .manage(config)
    .mount("/",routes![hello,sign_up,sign_in,get_all_contact,
    add_contact,update_contact,get_contact,delete_contact,google_sign_in])
}
