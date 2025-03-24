use std::sync::Arc;

use redis::AsyncCommands;

//Mark a  user as online in specific room using redis
pub async fn mark_user_online(
    user_id:i32,
    room_id:i32,
    redis:Arc<redis::Client>
)->redis::RedisResult<()>{
    //Get an async connection from redis connection pool
    let mut conn=redis.get_async_connection().await?;

    //Add user Id to Redis set named "room:{room_id}:online"
    //Using SADD command (Set add)
    conn.sadd(format!("room:{}:online",room_id), user_id).await
}

///Mark a user as offline in specific room using redis
pub async fn mark_user_offline(
    user_id:i32,
    room_id:i32,
    redis:Arc<redis::Client>
)->redis::RedisResult<()>{
    //Get an async connection from redis connection pool
    let mut conn=redis.get_async_connection().await?;

    //Add user Id to Redis set named "room:{room_id}:online"
    //Using srem command (Set add)
    conn.srem(format!("room:{}:online",room_id), user_id).await
}

///Retrive all online users for a specific room from redis
pub async fn get_online_users(
    room_id:i32,
    redis:Arc<redis::Client>
)->redis::RedisResult<()>{
let mut conn = redis.get_async_connection().await?;

 conn.smembers(format!("room:{}:online",room_id)).await

}