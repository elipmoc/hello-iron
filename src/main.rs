extern crate iron;
#[macro_use]
extern crate router;

mod controllers;
mod routes;

use iron::prelude::*;
use routes::create_router;
use std::env;

fn main() {
   run_server();
}

fn run_server(){
    let port = env::var("PORT").unwrap_or("3000".to_string());
    println!("serving on http://localhost:{}...", port);
    Iron::new(create_router())
        .http(format!("0.0.0.0:{}", port))
        .unwrap();
}
