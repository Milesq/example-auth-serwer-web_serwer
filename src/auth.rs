use bcrypt::{hash, DEFAULT_COST};
use std::{collections::HashMap, thread};
use web_server::{HttpCode, Request, Response};
mod register;

macro_rules! json_err {
    ($msg: expr) => {
        "{\"err\": \"$msg\"}"
    };
}

pub fn register(req: Request, mut res: Response) -> Response {
    let body = serde_urlencoded::from_str::<HashMap<String, String>>(req.get_body().as_str());

    if body.is_err() {
        res.set_http_code(HttpCode::_400).set_body("Incorrect body");
        return res;
    }

    let body: HashMap<String, String> = body.unwrap();
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

    thread::spawn(move || {
        let hashed = hash(pass, DEFAULT_COST).unwrap();
        let register_result = register::save_user(user.to_string(), hashed);

        log::trace!("Cannot register user");
        log::error!("{:#?}", register_result.unwrap_err());
    });

    res.set_body(crate::private_key().as_str());
    res
}

pub fn login(_req: Request, res: Response) -> Response {
    res
}
