use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    id: String,
    username: String,
}

impl User {
    pub fn new(username: String) -> User {
        User {
            id: Uuid::new_v4().simple().to_string(),
            username,
        }
    }
}
