use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::HashMap;

use web_server::{HttpCode, Request, Response};

pub fn auth_required(req: &Request, res: &Response) -> Result<String, Response> {
    let mut res = res.clone();

    let token = if let Some(token) = req.cookie("token") {
        token
    } else {
        res.set_body("Unauthorized").set_http_code(HttpCode::_401);
        return Err(res.clone());
    };

    let key: Hmac<Sha256> = Hmac::new_varkey(crate::private_key().as_bytes()).unwrap();

    let claims: HashMap<String, String> =
        VerifyWithKey::verify_with_key(token.as_str(), &key).unwrap();

    Ok(claims.get("user").unwrap().clone())
}
