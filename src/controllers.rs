use iron::prelude::*;
use iron::status;
use iron::mime::*;
use iron::modifiers::*;
use iron::Handler;
use std::sync::Mutex;
use std::cell::Cell;
use std::sync::Arc;
pub struct GetHello {
    count: Arc<Mutex<Cell<usize>>>
}

impl GetHello {
    pub fn new() -> GetHello {
        GetHello { count: Arc::new(Mutex::new(Cell::new(0))) }
    }
}

impl Handler for GetHello {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        let mut res = Response::new();
        res.set_mut(status::Ok);
        let mime = Mime(
            TopLevel::Text,
            SubLevel::Html,
            vec![(Attr::Charset, Value::Utf8)],
        );
        res.set_mut(mime);
        let new_count=self.count.lock().unwrap().get()+1;
        self.count.lock().unwrap().set(new_count);
        res.set_mut(format!("<h1>{}</h1>",self.count.lock().unwrap().get()));
        Ok(res)
    }
}

pub fn get_hello(_req: &mut Request) -> IronResult<Response> {
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

pub fn post_hello(_req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::Found);
    res.set_mut(RedirectRaw("/hello".to_string()));
    Ok(res)
}

pub fn no_page(_req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    res.set_mut(status::NotFound);
    let mime = Mime(
        TopLevel::Text,
        SubLevel::Html,
        vec![(Attr::Charset, Value::Utf8)],
    );
    res.set_mut(mime);
    res.set_mut(include_str!("../no_page.html"));
    Ok(res)
}