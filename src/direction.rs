use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Direction {
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