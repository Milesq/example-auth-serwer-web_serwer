use std::fs::{File, OpenOptions};

use super::Users;

pub fn get_users() -> Users {
    if let Ok(file) = File::open(&crate::users_file()) {
        bincode::deserialize_from::<_, Users>(file).unwrap_or_else(|_| {
            log::error!("Database format is incorrect");
            panic!();
        })
    } else {
        Vec::new()
    }
}

pub fn save_user(name: String, pass: String) -> bincode::Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(crate::users_file().as_str())
        .unwrap();
    let users = bincode::deserialize_from::<_, Users>(&file);

    let mut users = if let Ok(users) = users {
        users
    } else {
        Vec::new()
    };
    users.push((name, pass));

    bincode::serialize_into(file, &users)
}
