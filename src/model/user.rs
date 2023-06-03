use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub date_of_birth_ts: u32,
    created_at: u128
}

impl User {
    pub fn new(name: String, date_of_birth_ts: u32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name,
            date_of_birth_ts: date_of_birth_ts,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis(),
        }
    }
}