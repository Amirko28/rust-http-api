use crate::{model::user::User, data::get_data};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum UserError {
    UserCreation
}

pub fn get_users() -> Vec<User> {
    let mut users = get_data();
    users.sort_by(|first , second| first.created_at.cmp(&second.created_at));
    users
}

pub fn insert_user(name: String, date_of_birth_ts: u128) -> Result<User, crate::dal::users::UserError> {
    println!("Adding new user");
    Ok(User::new(name, date_of_birth_ts))
}