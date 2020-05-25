use crate::auth::auth_required;
// use cookie::Cookie;
use web_server::{Request, Response};

pub fn get_all(req: Request, mut res: Response) -> Response {
    let current_user = match auth_required(&req, &res) {
        Err(r) => return r,
        Ok(user) => user,
    };

    println!("{}", current_user);
    // res.set_header("Set-Cookie", &Cookie::new("adsadadasd", "sadd").to_string());

    res
}
