// Copyright (c) 2016 Patrick Burroughs <celti@celti.name>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>, or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

// Module declarations
//pub mod block;
pub mod braille;
pub mod graph;

// Crate-wide declarations
use std::{error, fmt, result};

pub type Graph     = Vec<Vec<u32>>;
pub type Result<T> = result::Result<T, ParseGraphError>;

#[derive(Debug,Clone,PartialEq)]
#[doc(hidden)]
enum GraphErrorKind {
    NoInput,
    InvalidChar,
}

#[derive(Debug,Clone,PartialEq)]
pub struct ParseGraphError {
    #[doc(hidden)]
    kind: GraphErrorKind
}

impl ParseGraphError {
    #[doc(hidden)]
    fn __description(&self) -> &str {
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
