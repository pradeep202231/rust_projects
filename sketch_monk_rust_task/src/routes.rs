use diesel::prelude::*;
use diesel::dsl::{count, sql};
use diesel::sql_types::{Nullable, Text, Array};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::db::{get_conn, PgPool};
use crate::error::{ErrorResponse, Response, SuccessResponse};
use crate::model::{CreatePostRequest, ListPostsResponse, NewPost, NewUser, PaginationMeta, Post, PostTag, PostWithTags, User, UserInfo};
use crate::schema::*;


#[post("/users", data="<user>")]
pub async fn create_user(pool:&State<PgPool>,
 user:Json<NewUser>)->Response<Json<User>>{
    let conn = &mut get_conn(pool);

    let exists = users::table
        .filter(users::username.eq(&user.username))
        .select(count(users::id))
        .first::<i64>(conn)
        .map_err(|e| ErrorResponse((Status::InternalServerError, e.to_string())))?;
    
    if exists > 0 {
        return Err(ErrorResponse((Status::Conflict, "Username already exists".to_string())));
    }

    diesel::insert_into(users::table)
        .values(&user.into_inner())
        .get_result::<User>(conn)
        .map(|user| SuccessResponse((Status::Created, Json(user))))
        .map_err(Into::into)
}

#[post("/users/<user_id>/posts", data= "<post_data>")]
pub async fn create_post(
    pool: &State<PgPool>,
    user_id:i32,
    post_data:Json<CreatePostRequest>
)->Response<Json<Post>>{
    let conn = &mut get_conn(pool);

    //Start the transaction
    let post=conn.transaction::<Post, diesel::result::Error, _>(|conn|{
        let new_post=NewPost{
            created_by:user_id,
            title:post_data.title.clone(),
            body:post_data.body.clone(),
        };

        let post:Post=diesel::insert_into(posts::table)
               .values(&new_post)
               .get_result(conn)?;

        //Insert all tags
        if !post_data.tags.is_empty() {
            let post_tags = post_data.tags.iter().map(|tag| PostTag {
                fk_post_id: post.id,
                tag: tag.clone(),
            }).collect::<Vec<_>>();

            diesel::insert_into(posts_tags::table)
                   .values(&post_tags)
                   .execute(conn)?;
        }

        Ok(post)
    }).expect("Error creating post with tags");

    
    Ok(SuccessResponse((Status::Ok,Json(post))))
}


#[get("/posts?<page>&<limit>&<search>")]
pub async fn list_posts(
    pool: &State<PgPool>,
    page: i64,
    limit:i64,
    search: Option<String>,
) -> Response<Json<ListPostsResponse>> {
    let conn = &mut get_conn(pool);
    // Calculate offset
    let offset = (page - 1) * limit;

    // First get the total count without any pagination
    let total_docs = {
        let mut count_query = posts::table.left_join(users::table).into_boxed();
        
        if let Some(search_term) = search.as_ref() {
            count_query = count_query.filter(
                posts::title.ilike(format!("%{}%", search_term))
                .or(posts::body.ilike(format!("%{}%", search_term))));
        }
        
        count_query
            .select(diesel::dsl::count_distinct(posts::id))
            .first::<i64>(conn)?
    };

    // Then build the query for fetching IDs with pagination
    let post_ids = {
        let mut ids_query = posts::table.left_join(users::table).into_boxed();
        
        if let Some(search_term) = search {
            ids_query = ids_query.filter(
                posts::title.ilike(format!("%{}%", search_term))
                .or(posts::body.ilike(format!("%{}%", search_term))));
        }
        
        ids_query
            .select(posts::id)
            .order_by(posts::created_at.desc())
            .offset(offset)
            .limit(limit)
            .load::<i32>(conn)?
    };

    // Rest of the function remains the same...
    // Main query to get posts with their user info
    let posts_with_users = posts::table
        .left_join(users::table)
        .filter(posts::id.eq_any(&post_ids))
        .select((
            posts::all_columns,
            users::id.nullable(),
            users::username.nullable(),
            users::first_name.nullable(),
            users::last_name.nullable(),
        ))
        .load::<(Post, Option<i32>, Option<String>, Option<String>, Option<String>)>(conn)?;

    // Get all tags for the posts in one query
    let tags_map = posts_tags::table
        .filter(posts_tags::fk_post_id.eq_any(&post_ids))
        .load::<PostTag>(conn)?
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut map, tag| {
            map.entry(tag.fk_post_id).or_insert_with(Vec::new).push(tag.tag);
            map
        });

    // Combine everything
    let posts_with_tags_and_users = posts_with_users
        .into_iter()
        .map(|(post, user_id, username, first_name, last_name)| {
            let tags = tags_map.get(&post.id).cloned().unwrap_or_default();
            
            let created_by = match (user_id, username, first_name, last_name) {
                (Some(id), Some(un), Some(r#fn), ln) => Some(UserInfo {
                    user_id: id,
                    username: un,
                    first_name: r#fn,
                    last_name: ln,
                }),
                _ => None,
            };

            PostWithTags {
                post,
                tags,
                created_by,
            }
        })
        .collect::<Vec<PostWithTags>>();

    // Calculate pagination metadata
    let total_pages = (total_docs as f64 / limit as f64).ceil() as i64;
    let from = offset + 1;
    let to = std::cmp::min(offset + limit, total_docs);

    Ok(SuccessResponse(
        (Status::Ok,
        Json(ListPostsResponse {
        records: posts_with_tags_and_users,
        meta: PaginationMeta {
            current_page: page,
            limit,
            from,
            to,
            total_pages,
            total_docs,
        },
    }
))))
}