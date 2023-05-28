use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
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
        bcrypt::verify(password, &self.password).unwrap()
    }

    pub fn new(username: String, password: String) -> User {
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        User {
            id: Uuid::new_v4().simple().to_string(),
            username,
            password: hashed_password,
        }
    }

    pub fn of(id: String, username: String, password: String) -> User {
        User {
            id,
            username,
            password,
        }
    }
}
