// Copyright (c) 2016 Patrick Burroughs <celti@celti.name>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>, or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

use super::error::Result;
use super::error::ParseGraphError as Error;
use super::error::GraphErrorKind::NoInput;

use std::cmp::max;

const EMPTY: u32 = 0x2800;
const CHAR: [&'static [&'static [u32]]; 4] =
    [ &[ &[0x00, 0x40, 0x44, 0x46, 0x47],
         &[0x00, 0x80, 0xA0, 0xB0, 0xB8],],
      &[ &[0x00, 0x01, 0x09],
         &[0x00, 0x02, 0x12],
         &[0x00, 0x04, 0x24],
         &[0x00, 0x40, 0xC0],],
      &[ &[0x00, 0x01, 0x03, 0x07, 0x47], 
         &[0x00, 0x08, 0x18, 0x38, 0xB8],],
      &[ &[0x00, 0x08, 0x09],
         &[0x00, 0x10, 0x12],
         &[0x00, 0x20, 0x24],
         &[0x00, 0x80, 0xC0],],
    ];

pub fn horizontal_graph(reverse: bool, input: Vec<usize>) -> Result<Vec<Vec<u32>>> {
    let blocks     = input.len() / 2 + if input.len() % 2 > 0 { 1 } else { 0 };
    let lines      = max((try!(input.iter().max().ok_or(Error { kind: NoInput })) + 3) / 4, 1);
    let mut output = vec![vec![EMPTY; blocks]; lines];
    let char       = if reverse { 2 } else { 0 };

    for (block,chunk) in input.chunks(2).collect::<Vec<&[usize]>>().iter().enumerate() {
        for (index,value) in chunk.iter().enumerate() {
            for line in 0..lines {
                let partial = value.saturating_sub(line * 4);
                let insert = if partial > 4 { 4 } else { partial };

                output[line][block] += CHAR[char][index][insert]
            }
        }
    };

    if !reverse {
        output.reverse()
    };

    Ok(output)
}

pub fn vertical_graph(reverse: bool, input: Vec<usize>) -> Result<Vec<Vec<u32>>> {
    let blocks     = max((try!(input.iter().max().ok_or(Error { kind: NoInput })) + 1) / 2, 1);
    let lines      = input.len() / 4 + if input.len() % 4 > 0 { 1 } else { 0 };
    let mut output = vec![vec![EMPTY; blocks]; lines];
    let char       = if reverse { 3 } else { 1 };

    for (line, chunk) in input.chunks(4).collect::<Vec<&[usize]>>().iter().enumerate() {
        for (index,value) in chunk.iter().enumerate() {
            for block in 0..blocks {
                let partial = value.saturating_sub(block * 2);
                let insert = if partial > 2 { 2 } else { partial };

                output[line][block] += CHAR[char][index][insert]
            }
        }
    };

    if reverse {
        for line in 0..lines {
            output[line].reverse()
        }
    };

    Ok(output)
}
