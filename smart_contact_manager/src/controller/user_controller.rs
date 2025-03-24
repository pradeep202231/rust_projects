use rocket::{State,serde::{Deserialize,Serialize}};
use sea_orm::*;
use crate::entities::{prelude::*, users};
use crate::controller::ErrorResponse;
use rocket::{http::Status,serde::json::Json};
use crate::controller::SuccessResponse;
use super::Response;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::AppConfig;
use crate::auth::Claims;
use std::time::SystemTime;
use jsonwebtoken::{encode,EncodingKey,Header};     

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    token: String,
}

#[post("/auth/signin", data = "<req_sign_in>")]
pub async fn sign_in(
    db: &State<DatabaseConnection>,
    config: &State<AppConfig>,
    req_sign_in: Json<ReqSignIn>,
) -> Response<Json<ResSignIn>> {
    let db = db as &DatabaseConnection;
    let config = config as &AppConfig;

    let u: users::Model = match User::find()
        .filter(users::Column::Username.eq(&req_sign_in.username))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            return Err(ErrorResponse((
                Status::Unauthorized,
                "Invalid credentials".to_string(),
            )))
        }
    };

    if !verify(&req_sign_in.password, &u.password).unwrap() {
        return Err(ErrorResponse((
            Status::Unauthorized,
            "Invalid credentials".to_string(),
        )));
    }

    let claims = Claims {
        sub: u.id,
        role: "user".to_string(),
        exp: (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60*60) as i64,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(SuccessResponse((Status::Ok, Json(ResSignIn { token }))))
}


#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate="rocket::serde")]
pub struct ReqSignUp{
    username:String,
    email:String,
    password:String,
}

#[post("/auth/signup",data="<req_sign_up>")]
pub async  fn sign_up(db:&State<DatabaseConnection>,req_sign_up:Json<ReqSignUp>)->Response<String>{
    println!("Received signup request: {:?}", req_sign_up); // Debugging request da
    println!("after 123 55");
    let db=db as &DatabaseConnection;
    println!("after  123 rr");
    if User::find()
    .filter(users::Column::Email.eq(&req_sign_up.email))
    .one(db)
    .await?
    .is_some(){
        return Err(ErrorResponse((
            Status::UnprocessableEntity,
            "An account exits with that email address.".to_string(),
        )))
    }
    println!("after  123");
    User::insert(users::ActiveModel {
        username:Set(req_sign_up.username.to_owned()),
        email: Set(req_sign_up.email.to_owned()),
        password: Set(hash(&req_sign_up.password,10).unwrap()),
        provider:Set("local".to_string()),
        ..Default::default()
    })
    .exec(db)
    .await?;
    println!("last 12345");
    Ok(SuccessResponse((
        Status::Created,
        "User registered successfully".to_string()
    )
    ))    
}