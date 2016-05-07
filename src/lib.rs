// Copyright (c) 2016 Patrick Burroughs <celti@celti.name>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>, or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

pub mod braille;

pub mod graph {
    use std::{char, error, fmt, result};

    #[derive(Debug)]
    pub struct ParseGraphError;

    pub type Result<T> = result::Result<T, ParseGraphError>;

    impl error::Error for ParseGraphError {
        fn description(&self) -> &str { "failed to parse graph" }
    }

    impl fmt::Display for ParseGraphError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parse error: failed to parse graph")
        }
    }

    pub fn graph_to_strings(input: Vec<Vec<u32>>) -> Result<Vec<String>> {
        let mut output = Vec::new();

        for (index,line) in input.iter().enumerate() {
            let mut string = String::new();

            for block in line {
                string.push(try!(char::from_u32(*block).ok_or(ParseGraphError)))
            };

            output.insert(index,string)
        };

        Ok(output)
    }
}
