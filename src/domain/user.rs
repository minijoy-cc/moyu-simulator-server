use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    password: String,
}

impl User {
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn login(&self, password: String) -> bool {
        password == String::from(&self.password)
    }

    pub fn new(username: String, password: String) -> User {
        User {
            id: Uuid::new_v4().simple().to_string(),
            username,
            password,
        }
    }
}
