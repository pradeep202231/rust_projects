use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{prelude::{Associations, Identifiable}, sql_types::{Array, Nullable, Text}, Insertable, Queryable, QueryableByName, Selectable};
use crate::schema::*;

#[derive(Queryable, Selectable, Serialize,Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub created_by: i32,
    pub title: String,
    pub body: String,
}


#[derive(Serialize)]
pub struct PaginatedResponse<T>{
    pub records:Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta{
    pub current_page:i64,
    pub limit:i64,
    pub from:i64,
    pub to:i64,
    pub total_pages:i64,
    pub total_docs:i64
}

#[derive(Insertable, Queryable, Identifiable, Debug)]
#[table_name = "posts_tags"]
#[primary_key(fk_post_id, tag)]
pub struct PostTag {
    pub fk_post_id: i32,
    pub tag: String,
}

#[derive(Deserialize)]
pub struct CreatePostRequest{
    pub title:String,
    pub body:String,
    pub tags:Vec<String> //New field for tags
}

#[derive(Deserialize,FromForm)]
pub struct PaginationParams{
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>
}

use diesel::prelude::*;

#[derive(Queryable, Selectable, QueryableByName, Debug, Serialize)] // <--- Added QueryableByName here
#[diesel(table_name = posts)]
pub struct Post {
    #[diesel(sql_type = diesel::sql_types::Int4)] 
    pub id: i32,

    #[diesel(sql_type = diesel::sql_types::Int4)]
    pub created_by: i32,

    #[diesel(sql_type = diesel::sql_types::VarChar)]
    pub title: String,

    #[diesel(sql_type = diesel::sql_types::Text)]
    pub body: String,

    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: chrono::NaiveDateTime,
}


// In your model.rs or appropriate file
#[derive(Queryable)]
pub struct PostWithAggregatedTags {
    pub id: i32,
    pub created_by: Option<i32>,
    pub title: String,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tag: Option<String>,
}


#[derive(Serialize)]
pub struct PostWithUserAndTags {
    #[serde(flatten)]
    pub post: Post,
    pub created_by: Option<User>,
    pub tags: Vec<String>,
}


#[derive(Queryable, Debug)]
pub struct PostWithAggregatedTagsAndUser {
    pub post: Post,
    pub user: Option<User>,
    pub tags: Vec<Option<String>>,
}


#[derive(Serialize)]
pub struct UserInfo {
    pub user_id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Serialize)]
pub struct ListPostsResponse {
    pub records: Vec<PostWithTags>,  // Changed from PostWithUserAndTags to PostWithTags
    pub meta: PaginationMeta,
}

// Keep your existing PostWithTags struct as is
#[derive(Serialize)]
pub struct PostWithTags {
    #[serde(flatten)]
    pub post: Post,
    pub tags: Vec<String>,
    pub created_by: Option<UserInfo>,
}