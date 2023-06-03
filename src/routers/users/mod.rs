use actix_web::{get, HttpResponse, http::header::ContentType, web, Scope};

use crate::business_logic;

#[get("")]
pub async fn get_all() -> HttpResponse {
    let users = business_logic::users::get_users();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(users)
}

pub fn users_router() -> Scope {
    let users_scope = web::scope("/users")
        .service(get_all);
    users_scope
}


