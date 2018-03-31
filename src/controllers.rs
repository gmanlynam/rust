use iron::Request;
use iron::IronResult;
use iron::Response;
use iron::status;
use iron::Set;

pub fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
<input type="text" name="n"/>
<button type="submit">Input commands</button>
</form>
"#);
    Ok(response)
}