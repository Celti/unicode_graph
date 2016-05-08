use super::{Graph, Result};
use super::GraphErrorKind::InvalidChar;
use super::ParseGraphError as Error;

pub fn graph_to_strings(input: Graph) -> Result<Vec<String>> {
    let mut output = Vec::with_capacity(input.len());

    for line in input.into_iter() {
        output.push(try!(
            line.into_iter()
            .map(|block| ::std::char::from_u32(block).ok_or(Error { kind: InvalidChar }))
            .collect()))
    };

    Ok(output)
}
