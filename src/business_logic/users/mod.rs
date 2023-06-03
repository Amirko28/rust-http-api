use crate::model::user::User;

pub fn get_users() -> Vec<User> {
    let data: Vec<User> = vec![User::new("Amir".into(), 1000)];

    data
}