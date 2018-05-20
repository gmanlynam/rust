
extern crate iron;
extern crate router;
#[macro_use]
extern crate log;
extern crate simple_logger;
#[macro_use] extern crate mime;

extern crate core;


use iron::prelude::*;
use router::Router;

use iron::Iron;
use command_processor::post_command;

mod command_processor;
mod direction;
mod controllers;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let mut router = Router::new();
    router.get("/", controllers::get_form , "form");
    router.post("/gcd", post_command, "gcd");
    let chain = Chain::new(router);

    info!("Serving on http://localhost:8080...");
    Iron::new(chain).http("localhost:8080").unwrap();
}