use actix_web::{get, HttpResponse, http::header::ContentType, web::{Query, scope, Json}, Scope, post};
use serde::Deserialize;

use crate::{business_logic, model::user::User};

#[derive(Deserialize)]
pub struct UserQueryParam {
    name: Option<String>
}

#[derive(Deserialize)]
pub struct NewUser {
    name: String,
    date_of_birth_ts: u128
}

#[get("")]
pub async fn get_all(query: Query<UserQueryParam>) -> HttpResponse {
    let users: Vec<User> = match &query.name {
        None => business_logic::users::get_all_users(),
        Some(query) => business_logic::users::find_user_by_name(query.clone())
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(users)
}

#[get("/{id}")]
pub async fn get_user_by_id(id: String) -> HttpResponse {
    let user = business_logic::users::get_user(id);
    match user {
        None => HttpResponse::NotFound().content_type(ContentType::json()).json({}),
        Some(us) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(us)
    }
}

#[post("")]
pub async fn create_user(new_user: Json<NewUser>) -> HttpResponse {
    let new_user = new_user.into_inner();
    let insert_result = business_logic::users::create_user(new_user.name, new_user.date_of_birth_ts);
    match insert_result {
        Ok(user) => HttpResponse::Ok().content_type(ContentType::json()).json(user),
        Err(e) => HttpResponse::InternalServerError().json(e)
    }
}

pub fn users_router() -> Scope {
    let users_scope = scope("/users")
        .service(get_all)
        .service(get_user_by_id)
        .service(create_user);
    users_scope
}
