// Copyright (c) 2016 Patrick Burroughs <celti@celti.name>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>, or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

extern crate unicode_graph;
use unicode_graph::graph::graph_to_strings;

#[test]
fn graph_to_strings_render_braille_full_eight_by_eight() {
    let input: Vec<Vec<u32>> = vec![vec![0x28FF; 4]; 2];
    let expect: Vec<String>  = vec!["⣿⣿⣿⣿".to_owned(); 2];
    assert!( graph_to_strings(input.clone()).unwrap().eq(&expect) );
}

#[test]
fn graph_to_strings_render_braille_incrementing_one_to_eight() {
    let input:  Vec<Vec<u32>> = vec![vec![0x2800, 0x2800, 0x28E0, 0x28FE], vec![0x28E0, 0x28FE, 0x28FF, 0x28FF]];
    let expect: Vec<String>   = vec!["⠀⠀⣠⣾".to_owned(), "⣠⣾⣿⣿".to_owned()];
    assert!( graph_to_strings(input.clone()).unwrap().eq(&expect) );
}


#[test]
fn graph_to_strings_render_braille_decrementing_eight_to_one() {
    let input:  Vec<Vec<u32>> = vec![vec![0x28F7, 0x28C4, 0x2800, 0x2800], vec![0x28FF, 0x28FF, 0x28F7, 0x28C4]];
    let expect: Vec<String>   = vec!["⣷⣄⠀⠀".to_owned(), "⣿⣿⣷⣄".to_owned()];
    assert!( graph_to_strings(input.clone()).unwrap().eq(&expect) );
}
