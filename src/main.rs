extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::*;
use std::env;

fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    println!("serving on http://localhost:{}...",port);
    Iron::new(get_hello)
        .http(format!("0.0.0.0:{}", port))
        .unwrap();
}

fn get_hello(_request: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::Ok);
    let mime = Mime(
        TopLevel::Text,
        SubLevel::Html,
        vec![(Attr::Charset, Value::Utf8)],
    );
    res.set_mut(mime);
    res.set_mut(include_str!("../hello.html"));
    Ok(res)
}
