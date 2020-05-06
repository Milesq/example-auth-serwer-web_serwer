use std::fs::OpenOptions;

type Users = Vec<(String, String)>;

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
