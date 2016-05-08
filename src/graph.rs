use super::error::Result;
use super::error::ParseGraphError as Error;
use super::error::GraphErrorKind::InvalidChar;

pub fn graph_to_strings(input: Vec<Vec<u32>>) -> Result<Vec<String>> {
    let mut output = Vec::new();

    for (index, line) in input.iter().enumerate() {
        let mut string = String::new();

        for block in line {
            string.push(try!(::std::char::from_u32(*block).ok_or(Error { kind: InvalidChar })))
        };

        output.insert(index, string)
    };

    Ok(output)
}
