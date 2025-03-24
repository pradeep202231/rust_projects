use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::{Deserialize, Serialize},
};
use chrono::Utc; // Add chrono for timestamp handling
use crate::AppConfig;
pub mod cors;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: i64, // Expiration time in seconds since the Unix epoch
}

pub struct AuthenticatedUser {
    pub id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Log the headers for debugging
        println!("Headers: {:?}", req.headers());

        // Extract the Authorization header
        if let Some(auth_header) = req.headers().get_one("Authorization") {
            println!("Authorization header: {}", auth_header);

            // Check if the header starts with "Bearer "
            if auth_header.starts_with("Bearer ") {
                // Extract the token (remove "Bearer " prefix)
                let token = auth_header.trim_start_matches("Bearer ").trim();
                println!("Token: {}", token);

                // Access the AppConfig state
                let config = match req.rocket().state::<AppConfig>() {
                    Some(config) => config,
                    None => return Outcome::Error((Status::InternalServerError, "AppConfig not found".to_string())),
                };

                // Decode the token
                let data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                    &Validation::new(jsonwebtoken::Algorithm::HS256),
                );

                match data {
                    Ok(token_data) => {
                        // Check if the token has expired
                        let current_time = Utc::now().timestamp();
                        if current_time > token_data.claims.exp {
                            // Token has expired
                            return Outcome::Error((Status::Unauthorized, "Token has expired".to_string()));
                        }

                        // Token is valid
                        println!("Decoded claims: {:?}", token_data.claims);
                        Outcome::Success(AuthenticatedUser { id: token_data.claims.sub })
                    }
                    Err(err) => {
                        println!("Token decode error: {:?}", err);
                        Outcome::Error((Status::Unauthorized, "Invalid token".to_string()))
                    }
                }
            } else {
                println!("Invalid Authorization header format");
                Outcome::Error((Status::Unauthorized, "Invalid Authorization header format".to_string()))
            }
        } else {
            println!("Authorization header absent");
            Outcome::Error((Status::Unauthorized, "Authorization header absent".to_string()))
        }
    }
}