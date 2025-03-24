use super::{ErrorResponse, Response, SuccessResponse};
use crate::{
    auth::AuthenticatedUser,
    entities::{contacts, prelude::*},
};
use rocket::serde::json::Json;
use rocket::State;
use rocket::{
    http::Status,
    serde::{Deserialize, Serialize},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, QueryFilter};
use sea_orm::EntityTrait;
use sea_orm::{DatabaseConnection, Set};
use sea_orm::ModelTrait;

#[derive(Debug)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResContact {
    contact_id: i32,
    name: String,
    email: String,
    phone_number: String,
    address: String,
    description: String,
}

impl From<&contacts::Model> for ResContact {
    fn from(c: &contacts::Model) -> Self {
        Self {
            contact_id: c.contact_id,
            name: c.name.clone(), // Cloning the String fields
            email: c.email.clone(),
            phone_number: c.phone_number.clone(),
            address: c.address.clone(),
            description: c.description.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ContactList {
    total: usize,
    contacts: Vec<ResContact>,
}

#[get("/contact")]
pub async fn get_all_contact(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser,
) -> Response<Json<Vec<ResContact>>> {
    let contacts = contacts::Entity::find()
        .all(db.inner())
        .await?
        .iter()
        .map(ResContact::from)
        .collect::<Vec<_>>();
   println!("cotacts sent {:?}",contacts);
    Ok(SuccessResponse((
        Status::Ok,
        Json(
            contacts,
        ),
    )))
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ReqContact {
    name: String,
    email: String,
    phoneNumber: String,
    address: String,
    description: String,
}

#[post("/addContact", data = "<req_contact>")]
pub async fn add_contact(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_contact: Json<ReqContact>,
) -> Response<Json<ResContact>> {
    let db = db as &DatabaseConnection;
    println!("add contact {:?}",req_contact);
    let contact = contacts::ActiveModel {
        // contact_id: Set(user.id),
        name: Set(req_contact.name.clone()),
        email: Set(req_contact.email.clone()),
        phone_number: Set(req_contact.phoneNumber.clone()),
        address: Set(req_contact.address.clone()),
        description: Set(req_contact.description.clone()),
        ..Default::default()
    };
    
    println!("contact activeMode : {:?}",contact);
    let new_contact = contact.insert(db).await.unwrap();

    Ok(SuccessResponse((
        Status::Created,
        Json(ResContact::from(&new_contact)),
    )))
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateReqContact {
    contact_id:i32,
    name: String,
    email: String,
    phone_number: String,
    address: String,
    description: String,
}


#[put("/contact", data = "<req_contact>")]
pub async fn update_contact(
    db: &State<DatabaseConnection>,
    user: AuthenticatedUser,
    req_contact: Json<UpdateReqContact>,
) -> Response<Json<ResContact>> {
    let db = db as &DatabaseConnection;
    
    let mut contact: contacts::ActiveModel=match contacts::Entity::find_by_id(req_contact.contact_id).one(db).await?{
        Some(a)=>a.into(),
        None=>{
          return  Err(ErrorResponse(( Status::NotFound,
            "No author found with the contact id.".to_string(),
        
    )))
    }
};
    contact.email=Set(req_contact.email.clone());
    contact.address=Set(req_contact.address.clone());
    contact.description=Set(req_contact.description.clone());
    contact.phone_number=Set(req_contact.phone_number.clone());
    
     let contact=contact.update(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResContact::from(&contact)),
    )))
}

#[get("/contacts/<id>")]
pub async fn get_contact(db:&State<DatabaseConnection>, _user: AuthenticatedUser,id:i32)->Response<Json<ResContact>>{
    let db=db as &DatabaseConnection;

    let contact=contacts::Entity::find_by_id(id).one(db).await?;

    match contact {
        Some(c) => Ok(SuccessResponse((
            Status::Ok,
            Json(ResContact::from(&c)), // Convert from &contacts::Model to ResContact
        ))),
        None => Err(ErrorResponse((
            Status::NotFound,
            format!("No contact found with the id: {}.", id),
        ))),
    }
}


#[delete("/contact/<email>")]
pub async fn delete_contact(db:&State<DatabaseConnection>,_user:AuthenticatedUser,email:String)->Response<String>{
    let db=db as &DatabaseConnection;

    // let contact=match contacts::Entity::find_by_email(email).one(db).await?{
    //     Some(a)=>a,
    //     None=>{
    //         return Err(ErrorResponse((
    //             Status::NotFound,
    //             format!("contact not found"),
    //         ))
    //         )
    //     }
    // };

    let contact=match contacts::Entity::find()
    .filter(contacts::Column::Email.eq(email.clone()))
    .one(db)
    .await
    .map_err(|e| ErrorResponse((Status::InternalServerError,e.to_string())))?{
        Some(a)=>a,
        None=>{
            return Err(ErrorResponse(
                (
                    Status::NotFound,
                    "Contact not found".to_string()
                )
            ))
        }
    };
    
    contact.delete(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        "contact not found".to_string()
    )))

}