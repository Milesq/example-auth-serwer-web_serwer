use web_server::{Request, Response};

pub fn get_all(req: Request, res: Response) -> Response {
    println!("{:#?}", req);
    res
}
