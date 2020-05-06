mod auth;
mod posts;
use dotenv::dotenv;
use simplelog::*;
use std::{env, fs::File};

fn private_key() -> String {
    env::var("SECRET").expect("Cannot find private key. Please declare it in .env file")
}

fn users_file() -> String {
    env::var("USER_FILE").expect("Cannot find users data file. Please declare it in .env file")
}

fn main() {
    dotenv().unwrap();
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("server.log").unwrap(),
        ),
    ])
    .unwrap();

    web_server::new()
        .post("/users/register", auth::register)
        .post("/users/login", auth::login)
        .get("/posts", posts::get_all)
        .not_found(|_, _| "".into())
        .launch(8080);
}
