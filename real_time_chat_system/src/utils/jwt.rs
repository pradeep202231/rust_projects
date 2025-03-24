use std::time::SystemTime;
use jsonwebtoken::{decode, encode, EncodingKey, Header,Validation,DecodingKey};
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

#[derive(Serialize,Deserialize)]
pub struct Claims{
    pub user_id: i32,
    pub exp: usize
}

pub fn create_token(user_id:i32)->String{
    let exp=SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() + 3600;

    let claims = Claims{
        user_id,
        exp:exp as usize
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

pub fn validate_token(token: &str) -> Option<i32> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).ok()?;
    Some(token_data.claims.user_id)
}