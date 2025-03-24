

#[cfg(test)]
mod tests {
    use crate::{api::{self, auth::{login, register, AuthRequest}}, utils::jwt::validate_token};
    use axum::{Json,Extension};
    use futures::SinkExt;
    use tokio_tungstenite::{connect_async, tungstenite};
    use url::Url;
    use super::*;
    use sqlx::PgPool;
    use bcrypt::verify;
    use futures_util::StreamExt;

    // #[tokio::test]
    // async fn test_register() {
    //     let pool = PgPool::connect("postgres://postgres:pradeep@localhost:5432/realtime_chat").await.unwrap();
    //     let request = AuthRequest {
    //         username: "testuser2".to_string(),
    //         password: "testpass2".to_string(),
    //     };

    //     let response: Json<crate::api::auth::AuthResponse> = register(Extension(pool), Json(request)).await.unwrap();
    //     assert!(validate_token(&response.token).is_some());
    // }

    // #[tokio::test]
    // async fn test_login() {
    //     let pool = PgPool::connect("postgres://postgres:pradeep@localhost:5432/realtime_chat").await.unwrap();
    //     let request = AuthRequest {
    //         username: "testuser2".to_string(),
    //         password: "testpass2".to_string(),
    //     };

    //     let response = login(Extension(pool), Json(request)).await.unwrap();
    //     assert!(validate_token(&response.token).is_some());
    // }

    #[tokio::test]
    async fn test_websocket(){
        use axum::extract::ws::Message;
        use tokio_tungstenite::tungstenite;
        use url::Url;
    
        let pool = PgPool::connect("postgres://postgres:pradeep@localhost:5432/realtime_chat").await.unwrap();
            let request = AuthRequest {
                username: "testuser2".to_string(),
                password: "testpass2".to_string(),
            };
    
            let response = login(Extension(pool), Json(request)).await.unwrap();
        let url = Url::parse(&format!("ws://localhost:3000/ws/1?token={}", response.token)).unwrap();
        
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await.unwrap();
        let (mut write, mut read) = ws_stream.split();
        
        // Test sending a message
        write.send(tungstenite::Message::Text("Hello".to_string())).await.unwrap();
    
        // Test receiving a message (this depends on your server's echo behavior)
        if let Some(Ok(msg)) = read.next().await {
            match msg {
                tungstenite::Message::Text(text) => {
                    println!("Received message: {}", text);
                    // Assert based on your expected behavior
                    // This might need adjustment based on what your server actually sends back
                    assert_eq!(text, "Hello");
                }
                _ => panic!("Unexpected message type"),
            }
        } else {
            panic!("No message received");
        }        
    }
}