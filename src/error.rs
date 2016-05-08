use std::{error, fmt, result};

pub type Result<T> = result::Result<T, ParseGraphError>;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseGraphError {
    #[doc(hidden)]
    pub kind: GraphErrorKind
}

#[derive(Debug,Clone,PartialEq)]
#[doc(hidden)]
pub enum GraphErrorKind {
    NoInput,
    InvalidChar,
}

impl ParseGraphError {
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        match self.kind {
            GraphErrorKind::NoInput => "cannot make graph from empty input",
            GraphErrorKind::InvalidChar => "invalid UTF-8 character found in input",
        }
    }
}

impl fmt::Display for ParseGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl error::Error for ParseGraphError {
    fn description(&self) -> &str {
        self.__description()
    }
}
