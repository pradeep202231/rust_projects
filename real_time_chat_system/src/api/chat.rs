use std::collections::HashMap;

use axum::{extract::{ws::{Message, WebSocket, WebSocketUpgrade},Path}, http::StatusCode, response::{IntoResponse, Response}, Extension
};
use futures_util::StreamExt;
use redis::{RedisResult,AsyncCommands,Commands};
use sqlx::{PgPool,query::Query};

use crate::utils::jwt::validate_token;

pub async fn websocket_handler(ws:WebSocketUpgrade,room_id:Path<i32>,
params:axum::extract::Query<HashMap<String,String>>,
redis_client:Extension<redis::Client>,
pool:Extension<PgPool>)->Response{
    //Validate JWT token
    dbg!();
    let token=params.get("token").unwrap();

    if let Some(user_id)=validate_token(token){
        dbg!();
        ws.on_upgrade(move |socket| handle_socket2(socket, room_id, user_id, redis_client,pool))
    }else{
        dbg!();
       (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}

async fn handle_socket2(
    mut socket: WebSocket,
    room:Path<i32>,
    user_id:i32,
    redis_client:Extension<redis::Client>,
    pool:Extension<PgPool>
){
     // Get a new Redis connection for publishing
     let mut publish_conn = redis_client.get_async_connection().await.unwrap();
    
     // Get a separate Redis connection for subscribing
     let subscribe_conn = redis_client.get_async_connection().await.unwrap();
     let mut pubsub = subscribe_conn.into_pubsub();
     pubsub.subscribe(format!("room:{}", &*room)).await.unwrap();
 
     // Convert pubsub into a message stream
     let mut redis_stream = pubsub.into_on_message();
 
     // Spawn a task to handle incoming Redis messages
     let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);
     tokio::spawn(async move {
         while let Some(msg) = redis_stream.next().await {
             if let Ok(text) = msg.get_payload::<String>() {
                dbg!(&text);
                 if tx.send(text).await.is_err() {
                     break;
                 }
             }
         }
     });
 
     // Main message handling loop
     tokio::spawn(async move {
         loop {
             tokio::select! {
                 // Handle messages from Redis
                 Some(text) = rx.recv() => {
                     if socket.send(axum::extract::ws::Message::Text(text)).await.is_err() {
                         break;
                     }
                 },
                 // Handle messages from WebSocket
                 Some(Ok(msg)) = socket.next() => {
                     if let axum::extract::ws::Message::Text(text) = msg {
                         let _: redis::RedisResult<()> = publish_conn
                             .publish(format!("room:{}", &*room), &text)
                             .await;
                         dbg!(&text);
                         // Optional: Store message in database
                         let _ = sqlx::query!(
                             "INSERT INTO messages (room_id, user_id, content) VALUES ($1, $2, $3)",
                             *room,
                             user_id,
                             text
                         )
                         .execute(&*pool)
                         .await;
                     }
                 }
             }
         }
     });
}