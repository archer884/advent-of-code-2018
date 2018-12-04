mod iter;

use std::error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

pub use self::iter::Units;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Unit {
    from_left: u16,
    from_top: u16,
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.from_left, self.from_top)
    }
}

#[derive(Debug)]
pub struct Claim {
    id: usize,
    from_left: u16,
    from_top: u16,
    width: u16,
    height: u16,
}

impl Claim {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn units(&self) -> Units {
        Units::new(self)
    }
}

#[derive(Parser)]
#[grammar = "../resource/grammar.pest"]
struct ClaimParser;

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pest::Parser;

        let tokens = ClaimParser::parse(Rule::Claim, s)
            .map_err(ParseClaimError::other)?
            .flatten();

        let mut id = None;
        let mut origin = None;
        let mut size = None;

        for token in tokens {
            match token.as_rule() {
                Rule::Id => id = Some(token.as_str()),
                Rule::Origin => origin = Some(token.as_str()),
                Rule::Size => size = Some(token.as_str()),

                // Skip...
                _ => (),
            }
        }

        let id = id.unwrap();
        let mut origin = origin.unwrap().split(",");
        let mut size = size.unwrap().split("x");

        Ok(Claim {
            id: id.parse()?,
            from_left: origin.next().unwrap().parse()?,
            from_top: origin.next().unwrap().parse()?,
            width: size.next().unwrap().parse()?,
            height: size.next().unwrap().parse()?,
        })
    }
}

#[derive(Debug)]
pub enum ParseClaimError {
    Integer(ParseIntError),
    Other(Box<error::Error>),
}

impl ParseClaimError {
    fn other(e: impl error::Error + 'static) -> ParseClaimError {
        ParseClaimError::Other(Box::new(e))
    }
}

impl From<ParseIntError> for ParseClaimError {
    fn from(e: ParseIntError) -> ParseClaimError {
        ParseClaimError::Integer(e)
    }
}

impl Display for ParseClaimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseClaimError::Integer(e) => write!(f, "Error parsing integer: {}", e),
            ParseClaimError::Other(e) => write!(f, "Error in parsing: {}", e),
        }
    }
}

impl error::Error for ParseClaimError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseClaimError::Integer(e) => Some(e),
            ParseClaimError::Other(e) => Some(e.as_ref()),
        }
    }
}
