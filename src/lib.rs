pub mod braille;

pub mod graph {
    use std::{fmt, error, result};

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
                string.push(try!(::std::char::from_u32(*block).ok_or(ParseGraphError)))
            };

            output.insert(index,string)
        };

        Ok(output)
    }
}
