use crate::domain::user::User;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::RwLock};

struct DB {
    id_2_users_mapping: HashMap<String, User>,
    username_2_id_mapping: HashMap<String, String>,
}

impl DB {
    fn exists(&self, name: &str) -> bool {
        self.username_2_id_mapping.contains_key(name)
    }

    fn save(&mut self, user: &User) -> bool {
        let name_inserted = self
            .username_2_id_mapping
            .insert(user.get_username().to_string(), user.get_id().to_string());

        let user_inserted = self
            .id_2_users_mapping
            .insert(user.get_id().to_string(), user.clone());
        name_inserted.is_none() && user_inserted.is_none()
    }

    fn find(&self, username: &str) -> Option<User> {
        self.username_2_id_mapping
            .get(username)
            .map(|id| self.id_2_users_mapping.get(id))
            .flatten()
            .map(|user| {
                User::of(
                    user.get_id().to_string(),
                    user.get_username().to_string(),
                    user.get_password().to_string(),
                )
            })
    }
}

lazy_static! {
    static ref LOCK: RwLock<DB> = RwLock::new(DB {
        id_2_users_mapping: HashMap::new(),
        username_2_id_mapping: HashMap::new()
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
