use serde::{Serialize,Deserialize};
use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;
use bcrypt::{hash, verify};
use crate::utils::jwt::create_token;

#[derive(Serialize,Deserialize)]
pub struct AuthRequest{
    pub username:String,
    pub password:String
}

#[derive(Serialize,Debug)]
pub struct AuthResponse{
    pub token:String
}

pub async fn register(pool: Extension<PgPool>,payload:Json<AuthRequest>)->Result<Json<AuthResponse>,StatusCode>{
    let password_hash=hash(&payload.password,10).unwrap();

    let user=sqlx::query!("INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id", payload.username, password_hash)
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token=create_token(user.id);
    Ok(Json(AuthResponse{token}))

}

pub async fn login(pool:Extension<PgPool>,payload:Json<AuthRequest>)->Result<Json<AuthResponse>, StatusCode>{
    let user=sqlx::query!("SELECT id, password_hash FROM users WHERE username = $1", payload.username)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    if verify(&payload.password, &user.password_hash).unwrap(){
        let token=create_token(user.id);
        Ok(Json(AuthResponse { token }))
    }else{
        Err(StatusCode::UNAUTHORIZED)
    }
}