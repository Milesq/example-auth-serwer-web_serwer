use crate::auth::auth_required;
use json::object;
use std::fs::{read_dir, read_to_string};
use web_server::{Request, Response};

pub fn get_all(req: Request, res: Response) -> Response {
    let _current_user = match auth_required(&req, &res) {
        Err(r) => return r,
        Ok(user) => user,
    };

    println!("{}", _current_user);

    let files = read_dir("./posts")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let files = files
        .iter()
        .map(|src| {
            object! {
                title: src.file_name().unwrap().to_str().unwrap(),
                content: read_to_string(src).unwrap()
            }
        })
        .collect::<Vec<_>>();

    (object! {
        data: {
            files: files
        }
    })
    .dump()
    .into()
}
