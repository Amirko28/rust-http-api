use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub date_of_birth_ts: u128,
    pub created_at: u128
}

impl From<&User> for User {
    fn from(user: &User) -> Self {
        let user = user.clone();
        User::new(String::from(user.name), user.date_of_birth_ts)
    }
}

impl User {
    pub fn new(name: String, date_of_birth_ts: u128) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name,
            date_of_birth_ts: date_of_birth_ts,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis(),
        }
    }
}