use crate::domain::user::User;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::RwLock;

struct DB {
    users: HashMap<String, User>,
    names: HashSet<String>,
}

impl DB {
    fn exists(&self, name: &str) -> bool {
        self.names.contains(name)
    }

    fn save(&mut self, user: &User) -> bool {
        let name_inserted = self.names.insert(user.get_name().to_string());
        let user_inserted = self.users.insert(user.get_id().to_string(), user.clone());
        name_inserted && user_inserted.is_none()
    }
}

lazy_static! {
    static ref LOCK: RwLock<DB> = RwLock::new(DB {
        users: HashMap::new(),
        names: HashSet::new()
    });
}

pub fn save(user: User) -> Result<User, String> {
    let mut db = LOCK.write().unwrap();

    if db.exists(user.get_name()) {
        return Err(String::from("用户名已存在"));
    }

    if db.save(&user) {
        Ok(user)
    } else {
        Err(String::from("创建用户失败"))
    }
}
