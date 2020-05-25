use cookie::Cookie;
use web_server::{Request, Response};

pub fn get_all(req: Request, mut res: Response) -> Response {
    println!("{:#?}", req);
    res.set_header("Set-Cookie", &Cookie::new("adsadadasd", "sadd").to_string());

    res
}
