use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn distance(self, Point { x, y }: Point) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

#[derive(Debug)]
pub enum ParsePointError {
    Integer(ParseIntError),
    Other(&'static str),
}

impl From<ParseIntError> for ParsePointError {
    fn from(e: ParseIntError) -> ParsePointError {
        ParsePointError::Integer(e)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(", ").map(|x| x.parse());

        let x = parts.next().ok_or(ParsePointError::Other("Bad format"))??;
        let y = parts.next().ok_or(ParsePointError::Other("Bad format"))??;

        if parts.next().is_some() {
            return Err(ParsePointError::Other("Bad format"));
        }

        Ok(Point { x, y })
    }
}

impl Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsePointError::Integer(e) => write!(f, "Failed to parse integer: {}", e),
            ParsePointError::Other(message) => f.write_str(message),
        }
    }
}

impl Error for ParsePointError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParsePointError::Integer(e) => Some(e),
            ParsePointError::Other(_) => None,
        }
    }
}
