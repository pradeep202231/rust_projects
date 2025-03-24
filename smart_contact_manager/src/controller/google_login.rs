use rocket::http::Status;
use rocket::{post, State, serde::json::Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::SystemTime;
use crate::entities::{prelude::*, users};
use crate::controller::{ErrorResponse, SuccessResponse, Response};
use crate::AppConfig;
use crate::auth::Claims;
use sea_orm::*;
use rocket::serde::{Deserialize, Serialize};

use super::token_verification::verify_google_token;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqGoogleSignIn {
    token: String, // Google ID token
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResGoogleSignIn {
    token: String, // Your application's JWT token
}

#[post("/api/auth/google", data = "<req_google_sign_in>")]
pub async fn google_sign_in(
    db: &State<DatabaseConnection>,
    config: &State<AppConfig>,
    req_google_sign_in: Json<ReqGoogleSignIn>,
) -> Response<Json<ResGoogleSignIn>> {
    let db = db as &DatabaseConnection;
    let config = config as &AppConfig;

    // Verify Google token
    let (email, name) = verify_google_token(&req_google_sign_in.token)
        .await
        .map_err(|e| ErrorResponse((Status::Unauthorized, e)))?;

    // Check if the user exists
    let user: users::Model = match User::find()
        .filter(users::Column::Email.eq(&email))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            // Create a new user if not found
            let new_user = users::ActiveModel {
                username: Set(name),
                email: Set(email),
                password: Set("".to_string()), // No password for Google users
                provider: Set("google".to_string()),
                ..Default::default()
            };
            let result = User::insert(new_user).exec(db).await?;
            users::Entity::find_by_id(result.last_insert_id)
                .one(db)
                .await?
                .ok_or(ErrorResponse((
                    Status::InternalServerError,
                    "Failed to create user".to_string(),
                )))?
        }
    };

    // Generate a JWT token
    let claims = Claims {
        sub: user.id,
        role: "user".to_string(),
        exp: (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60 * 60) as i64, // 1 hour expiration
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| ErrorResponse((Status::InternalServerError, e.to_string())))?;

    Ok(SuccessResponse((Status::Ok, Json(ResGoogleSignIn { token }))))
}