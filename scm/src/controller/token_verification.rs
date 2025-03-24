use reqwest::Client;
use serde_json::Value;

pub async fn verify_google_token(token: &str) -> Result<(String, String), String> {
    let client = Client::new();
    let response = client
        .get(&format!(
            "https://oauth2.googleapis.com/tokeninfo?id_token={}",
            token
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("Invalid Google token".to_string());
    }

    // Deserialize the response body into a `serde_json::Value`
    let payload: Value = response
        .json::<Value>()
        .await
        .map_err(|e| e.to_string())?;

    let email = payload["email"]
        .as_str()
        .ok_or("Missing email in Google token")?
        .to_string();
    let name = payload["name"]
        .as_str()
        .ok_or("Missing name in Google token")?
        .to_string();

    Ok((email, name))
}