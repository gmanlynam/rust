use iron::Request;
use iron::IronResult;
use iron::Response;
use iron::status;
use iron::Set;
use std::fs::File;
use std::io::prelude::*;

pub fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));

    let mut f = File::open("/Users/glynam/Development/rust/rust/src/html/webform.html").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    response.set_mut(contents);
    Ok(response)
}