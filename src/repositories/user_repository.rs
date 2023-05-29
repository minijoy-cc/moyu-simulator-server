use crate::domain::user::User;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::RwLock};

struct DB {
    users: HashMap<String, String>,
    usernames: HashMap<String, String>,
}

impl DB {
    fn exists(&self, name: &str) -> bool {
        self.usernames.contains_key(name)
    }

    fn save(&mut self, user: &User) -> bool {
        let name_inserted = self
            .usernames
            .insert(user.get_username().to_string(), user.get_id().to_string());

        let user_inserted = self.users.insert(
            user.get_id().to_string(),
            serde_json::to_string(&user).unwrap(),
        );
        name_inserted.is_none() && user_inserted.is_none()
    }

    fn find(&self, username: &str) -> Option<User> {
        self.usernames
            .get(username)
            .map(|id| self.users.get(id))
            .flatten()
            .map(|user_json_str| {
                let result = serde_json::from_str(user_json_str);
                result.unwrap()
            })
    }
}

lazy_static! {
    static ref LOCK: RwLock<DB> = RwLock::new(DB {
        users: HashMap::new(),
        usernames: HashMap::new()
    });
}

pub fn save(user: User) -> Result<User, String> {
    let mut db = LOCK.write().unwrap();

    if db.exists(user.get_username()) {
        return Err(String::from("用户名已存在"));
    }

    if db.save(&user) {
        Ok(user)
    } else {
        Err(String::from("创建用户失败"))
    }
}

pub fn find(username: &str) -> Option<User> {
    let db = LOCK.read().unwrap();
    db.find(username)
}
