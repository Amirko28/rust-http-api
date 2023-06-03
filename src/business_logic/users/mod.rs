use crate::{model::user::User, dal::users::{get_users, insert_user, UserError}};

pub fn get_all_users() -> Vec<User> {
    get_users()
}

pub fn get_user(id: String) -> Option<User> {
    let users: Vec<User> = get_users();
    let found_user = users.iter().find(|user| user.id == id).map(|us| us.into());
    found_user
}

pub fn find_user_by_name(name: String) -> Vec<User> {
    let users: Vec<User> = get_users();
    let found_user = users.iter().filter(|user| user.name.to_lowercase().contains(&name.to_lowercase())).map(|us| us.into()).collect();
    found_user
}

pub fn create_user(name: String, date_of_birth_ts: u128) -> Result<User, UserError> {
    insert_user(name, date_of_birth_ts)
}