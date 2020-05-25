use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, thread};
use web_server::{HttpCode, Request, Response};

mod register;

macro_rules! json_err {
    ($msg: tt) => {
        concat!("{\"err\": \"", $msg, "\"}")
    };
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct User {
    name: String,
}

pub fn register(req: Request, mut res: Response) -> Response {
    let body = serde_urlencoded::from_str::<HashMap<String, String>>(req.get_body().as_str());

    if body.is_err() {
        res.set_http_code(HttpCode::_400).set_body("Incorrect body");
        return res;
    }

    let body_arc: Arc<HashMap<String, String>> = Arc::new(body.unwrap());
    let body = Arc::clone(&body_arc);

    let (user, pass) = (body.get("name".into()), body.get("pass".into()));

    if user.is_none() || pass.is_none() {
        res.set_http_code(HttpCode::_400)
            .set_body(json_err!("User name or password wasn't provided"));
        return res;
    }

    let (user, pass) = (user.unwrap(), pass.unwrap());

    if user.len() < 4 || pass.len() < 6 {
        res.set_http_code(HttpCode::_400)
            .set_body(json_err!("User name or password is too short"));
        return res;
    }

    let body = Arc::clone(&body_arc);
    thread::spawn(move || -> Option<i32> {
        let user = body.get("name".into())?;
        let pass = body.get("pass".into())?;

        let hashed = hash(pass, DEFAULT_COST).unwrap();
        let register_result = register::save_user(user.to_string(), hashed);

        if let Err(e) = register_result {
            log::trace!("Cannot register user");
            log::error!("{:#?}", e);
        }

        None
    });

    res.set_body("JWT")
}

pub fn login(_req: Request, res: Response) -> Response {
    res
}
