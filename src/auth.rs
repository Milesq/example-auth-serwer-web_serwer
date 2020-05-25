use bcrypt::{hash, verify, DEFAULT_COST};
use cookie::Cookie;
use hmac::{Hmac, Mac};
use json::object;
use jwt::SignWithKey;
use sha2::Sha256;
use std::{collections::HashMap, sync::Arc, thread};
use web_server::{HttpCode, Request, Response};

mod users;

macro_rules! json_err {
    ($msg: tt) => {
        object! {
            err: $msg
        }.dump().as_str()
    };
}

type Users = Vec<(String, String)>;

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

    let users = users::get_users();
    let existing_user = users.iter().find(|&el| &el.0 == user);

    if existing_user.is_some() {
        return json_err!("User already exists!").into();
    }

    let body = Arc::clone(&body_arc);
    thread::spawn(move || -> Option<i32> {
        let user = body.get("name".into())?;
        let pass = body.get("pass".into())?;

        let hashed = hash(pass, DEFAULT_COST).unwrap();
        let register_result = users::save_user(user.to_string(), hashed);

        if let Err(e) = register_result {
            log::trace!("Cannot register user");
            log::error!("{:#?}", e);
        }

        None
    });

    let jwt_token = token(user);
    let jwt_token = jwt_token.as_str();

    let token_body = (object! {
        "data": {
            "token": jwt_token
        }
    })
    .dump();

    res.set_header("Set-Cookie", &Cookie::new("token", jwt_token).to_string())
        .set_body(&token_body);
    res
}

pub fn login(req: Request, mut res: Response) -> Response {
    let body = serde_urlencoded::from_str::<HashMap<String, String>>(req.get_body().as_str());

    let body = if let Ok(body) = body {
        body
    } else {
        res.set_http_code(HttpCode::_400).set_body("Incorrect body");
        return res;
    };

    let (user, pass) = (body.get("name".into()), body.get("pass".into()));

    if user.is_none() || pass.is_none() {
        res.set_http_code(HttpCode::_400)
            .set_body(json_err!("User name or password wasn't provided"));
        return res;
    }

    let (user, pass) = (user.unwrap(), pass.unwrap());

    let users = users::get_users();
    let existing_user = if let Some(user) = users.iter().find(|&el| &el.0 == user) {
        user
    } else {
        res.set_http_code(HttpCode::_400)
            .set_body(json_err!("User doesn't exists"));
        return res;
    };

    if verify(pass, &existing_user.1).unwrap() {
        let jwt_token = token(user);
        let jwt_token = jwt_token.as_str();

        let token_body = (object! {
            "data": {
                "token": jwt_token
            }
        })
        .dump();

        res.set_header("Set-Cookie", &Cookie::new("token", jwt_token).to_string())
            .set_body(&token_body);
    } else {
        res.set_body(json_err!("Password is incorrect"));
    }
    res
}

pub fn token(user: &str) -> String {
    let key: Hmac<Sha256> = Hmac::new_varkey(crate::private_key().as_bytes()).unwrap();
    let mut claims = HashMap::new();
    claims.insert("user", user);

    claims.sign_with_key(&key).unwrap()
}
