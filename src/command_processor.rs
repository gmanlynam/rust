extern crate iron;
extern crate router;
extern crate simple_logger;
extern crate urlencoded;

use self::urlencoded::UrlEncodedBody;
use iron::prelude::*;
use iron::status;
use direction::Direction;


fn process_command(x : i32, y: i32, commands : Vec<char>, mut direction: Direction) -> (i32, i32, Direction) {
        let mut cord = x;
        let mut lefty = y;
        commands.iter().for_each(|s| match *s {
            'f' => {
                match direction {
                    Direction::North => { cord += 1 },
                    Direction::West => { lefty -= 1 },
                    Direction::South => { cord -= 1 },
                    Direction::East => { lefty += 1 }
                }
            },
            'b' => {
                match direction {
                    Direction::North => { cord -= 1 },
                    Direction::West => { lefty += 1 },
                    Direction::South => { cord += 1 },
                    Direction::East => { lefty -= 1 }
                }
            },
            'l' => {
                match direction {
                    Direction::North => { direction =  Direction::West },
                    Direction::West => { direction =   Direction::South },
                    Direction::South => { direction =   Direction::East },
                    Direction::East => { direction =  Direction::North }
                }
            },
            'r' => {
                match direction {
                    Direction::North => { direction =   Direction::East },
                    Direction::West => { direction =   Direction::North },
                    Direction::South => { direction =   Direction::West },
                    Direction::East => { direction =   Direction::South }
                }
            },
            _ => error!("Received an invalid command")
        });
        return (cord, lefty, direction);
}

pub fn post_command(request: &mut Request) -> Result<Response, IronError> {
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

    let body = format!("{{\"forward\": {}, \"left\": {}, \"direction\": \"{}\"}}",
                       result.0, result.1, result.2);
    response.set_mut(status::Ok);
    response.set_mut(mime!(Application/Json));
    response.set_mut(body);
    Ok(response)

}

#[cfg(test)]
mod test_rover {
    use command_processor::*;
    use direction::Direction;
    #[test]
    fn test_rover() {
        assert_eq!(process_command(0i32, 0i32, vec!['f', 'f', 'f'], Direction::North), (3i32, 0i32, Direction::North));
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
}

