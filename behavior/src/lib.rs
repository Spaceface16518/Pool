use std::{fmt, io};
use std::io::{Error, Read};
use std::str::Utf8Error;

mod pest_parser;

pub trait Parser {
    type Output: EventHandler;

    fn parse(&self, src: &[u8]) -> Result<Self::Output, ParseError>;

    fn parse_from_reader<R: Read>(&self, reader: &mut R, size_hint: Option<usize>) -> Result<Self::Output, ParseError> {
        let mut bytes = size_hint.map(|hint| Vec::with_capacity(hint)).unwrap_or_default();
        reader.read_to_end(&mut bytes)?;
        self.parse(&bytes)
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ParseError {
    InvalidEncoding(Utf8Error),
    IOError(io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for ParseError {}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IOError(err)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> Self {
        ParseError::InvalidEncoding(err)
    }
}

pub trait EventHandler {
    fn react<I>(&self, conditions: Conditions) -> I where I: Iterator<Item=Action>;
}

#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
pub struct Conditions {
    pub near_strangers: bool,
    pub near_family: bool,
    pub hungry: bool,
    pub closest_stranger_is_larger: bool,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Action {
    Follow {
        target: Target
    },
    Flee {
        target: Target
    },
    Divide,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Target {
    Siblings,
    Strangers,
    Food,
}