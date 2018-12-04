use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp {
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug)]
pub enum ParseTimestampError {
    Integer(ParseIntError),
    Other(&'static str),
}

impl From<ParseIntError> for ParseTimestampError {
    fn from(e: ParseIntError) -> ParseTimestampError {
        ParseTimestampError::Integer(e)
    }
}

impl Display for ParseTimestampError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseTimestampError::Integer(e) => write!(f, "{}", e),
            ParseTimestampError::Other(message) => f.write_str(message),
        }
    }
}

impl Error for ParseTimestampError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseTimestampError::Integer(e) => Some(e),
            _ => None,
        }
    }
}

impl FromStr for Timestamp {
    type Err = ParseTimestampError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 16 {
            return Err(ParseTimestampError::Other("Invalid: wrong length"));
        }

        Ok(Timestamp {
            month: s[5..7].parse()?,
            day: s[8..10].parse()?,
            hour: s[11..13].parse()?,
            minute: s[14..16].parse()?,
        })
    }
}
