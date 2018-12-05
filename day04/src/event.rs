use crate::timestamp::{ParseTimestampError, Timestamp};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Event {
    timestamp: Timestamp,
    kind: EventKind,
}

#[derive(Copy, Clone, Debug)]
pub enum EventKind {
    Shift(u16),
    Wake,
    Sleep,
}

impl Event {
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn kind(&self) -> EventKind {
        self.kind
    }
}

#[derive(Debug)]
pub enum ParseEventError {
    Integer(ParseIntError),
    Other(&'static str),
    Timestamp(ParseTimestampError),
}

impl From<ParseTimestampError> for ParseEventError {
    fn from(e: ParseTimestampError) -> ParseEventError {
        ParseEventError::Timestamp(e)
    }
}

impl From<ParseIntError> for ParseEventError {
    fn from(e: ParseIntError) -> ParseEventError {
        ParseEventError::Integer(e)
    }
}

impl FromStr for Event {
    type Err = ParseEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s.as_bytes()[19] {
            b'w' => EventKind::Wake,
            b'f' => EventKind::Sleep,
            b'G' => {
                let haystack = &s[26..];
                let limit = haystack
                    .find(' ')
                    .ok_or(ParseEventError::Other("Seriously, now..."))?;

                EventKind::Shift(haystack[..limit].parse()?)
            }

            _ => return Err(ParseEventError::Other("Wtf?")),
        };

        Ok(Event {
            timestamp: s[1..17].parse()?,
            kind,
        })
    }
}
