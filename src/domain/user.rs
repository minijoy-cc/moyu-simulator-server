use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct User {
    id: String,
    name: String,
}

impl User {
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn new(name: String) -> User {
        User {
            id: Uuid::new_v4().simple().to_string(),
            name,
        }
    }
}
