
extern crate iron;
extern crate router;
#[macro_use]
extern crate log;
extern crate simple_logger;
#[macro_use] extern crate mime;

extern crate urlencoded;


use urlencoded::UrlEncodedBody;
use std::fmt;
use iron::prelude::*;
use iron::status;
use router::Router;
use log::{Log};
fn main() {

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_command, "gcd");
    let mut chain = Chain::new(router);

    error!("Serving on http://localhost:3000...");
    Iron::new(chain).http("localhost:3000").unwrap();

    let grid_size = [3,2];

    let x = 0i32;
    let y = 0i32;
    let facing = Direction::West;

    let commands =  vec!['f','f','b','l','l','f'];

    let (cord, lefty, direction) = process_command(x,y, commands, facing);
    info!("The total spaces moved was: {} forward, {} left, facing {}", cord % grid_size[0], lefty % grid_size[1], direction);
}

fn process_command(x : i32, y: i32, commands : Vec<char>, mut direction: Direction) -> (i32, i32, Direction) {
    let mut cord = x;
    let mut lefty = y;
    commands.iter().for_each(|s| match *s {
        'f' => {
            match direction {
                Direction::North => {cord += 1},
                Direction::West => {lefty += 1},
                Direction::South => {cord -= 1},
                Direction::East => {lefty -= 1}
            }},
        'b' => {
            match direction {
                Direction::North => {cord -= 1},
                Direction::West => {lefty -= 1},
                Direction::South => {cord += 1},
                Direction::East => {lefty += 1}
            }},
        'l' => {
            match direction {
            Direction::North => { direction = Direction::West},
            Direction::West => { direction = Direction::South},
            Direction::South => { direction = Direction::East},
            Direction::East => { direction = Direction::North}
            }},
        'r' => {
            match direction {
                Direction::North => { direction = Direction::East},
                Direction::West => { direction = Direction::North},
                Direction::South => { direction = Direction::West},
                Direction::East => { direction = Direction::South}
            }},
        _ => error!("Received an invalid command")
    });
    return (cord, lefty, direction) ;
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
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

fn post_command(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    info!("Received a call to process command");

    let command = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    ;
    let unparsed_commands = match command.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(commands) => commands
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_commands {
        for c in unparsed.chars() {
            numbers.push(c);
        }
    }

    let result = process_command(0,0, numbers, Direction::North);

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The rover has moved forward {}, left {} and is facing {}\n",
                result.0, result.1, result.2));
    Ok(response)

}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       let printable = match *self {
            Direction::North => "North",
            Direction::West => "West",
            Direction::South => "South",
            Direction::East => "East"
        };
        write!(f, "{}", printable)
    }
}

#[test]
fn test_rover() {
    assert_eq!(process_command(0i32,0i32, vec!['f','f','f'], Direction::North), (3i32,0i32, Direction::North));
}

#[test]
fn test_rover_changes_direction() {
    let comm = process_command(0i32, 0i32, vec!['l'], Direction::North);
    assert_ne!(comm.2, Direction::North);
}

#[test]
fn test_rover_goes_in_correct_direction() {
    let comm = process_command(0i32, 0i32, vec!['l', 'r', 'r', 'r', 'r', 'r'], Direction::North);
    assert_eq!(comm.2, Direction::North);
}
