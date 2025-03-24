use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders, Cors, CorsOptions};

pub fn cors_options() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::all(), // Allow all origins (or specify specific ones)
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete,Method::Put,Method::Options]
            .into_iter()
            .map(From::from)
            .collect(), // Allow GET, POST, and OPTIONS requests
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]), // Allow specific headers
        allow_credentials: true, // Allow credentials (e.g., cookies)
        ..Default::default()
    }
    .to_cors()
    .expect("CORS config failed")
}
