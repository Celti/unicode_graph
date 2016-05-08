// Copyright (c) 2016 Patrick Burroughs <celti@celti.name>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>, or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

extern crate unicode_graph;
use unicode_graph::Graph;
use unicode_graph::braille::{vertical_graph, horizontal_graph};

#[test]
fn vertical_graph_full_eight_by_eight() {
    let input:  Vec<usize> = vec![8; 8];
    let expect: Graph      = vec![vec![0x28FF; 4]; 2];
    assert!( vertical_graph(false, input.clone()).unwrap().eq(&expect) );
    assert!( vertical_graph(true,  input.clone()).unwrap().eq(&expect) );
}

#[test]
fn vertical_graph_empty_eight_by_eight() {
    let input:  Vec<usize> = vec![0; 8];
    let expect: Graph      = vec![vec![0x2800]; 2];
    assert!( vertical_graph(false, input.clone()).unwrap().eq(&expect) );
    assert!( vertical_graph(true,  input.clone()).unwrap().eq(&expect) );
}

#[test]
fn vertical_graph_incrementing_one_to_eight() {
    let input:    Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let expect:   Graph      = vec![vec![0x28F7, 0x28C4, 0x2800, 0x2800], vec![0x28FF, 0x28FF, 0x28F7, 0x28C4]];
    let expect_r: Graph      = vec![vec![0x2800, 0x2800, 0x28E0, 0x28FE], vec![0x28E0, 0x28FE, 0x28FF, 0x28FF]];
    assert!( vertical_graph(false, input.clone()).unwrap().eq(&expect)   );
    assert!( vertical_graph(true,  input.clone()).unwrap().eq(&expect_r) );
}

#[test]
fn vertical_graph_decrementing_eight_to_one() {
    let input:    Vec<usize> = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let expect:   Graph      = vec![vec![0x28FF, 0x28FF, 0x287F, 0x280B], vec![0x287F, 0x280B, 0x2800, 0x2800]];
    let expect_r: Graph      = vec![vec![0x2819, 0x28BF, 0x28FF, 0x28FF], vec![0x2800, 0x2800, 0x2819, 0x28BF]];
    assert!( vertical_graph(false, input.clone()).unwrap().eq(&expect)   );
    assert!( vertical_graph(true,  input.clone()).unwrap().eq(&expect_r) );
}

#[test]
fn horizontal_graph_full_eight_by_eight() {
    let input:  Vec<usize> = vec![8; 8];
    let expect: Graph      = vec![vec![0x28FF; 4]; 2];
    assert!( horizontal_graph(false, input.clone()).unwrap().eq(&expect) );
    assert!( horizontal_graph(true,  input.clone()).unwrap().eq(&expect) );
}

#[test]
fn horizontal_graph_empty_eight_by_eight() {
    let input:  Vec<usize> = vec![0; 8];
    let expect: Graph      = vec![vec![0x2800; 4]; 1];
    assert!( horizontal_graph(false, input.clone()).unwrap().eq(&expect) );
    assert!( horizontal_graph(true,  input.clone()).unwrap().eq(&expect) );
}

#[test]
fn horizontal_graph_incrementing_one_to_eight() {
    let input:    Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let expect:   Graph      = vec![vec![0x2800, 0x2800, 0x28E0, 0x28FE], vec![0x28E0, 0x28FE, 0x28FF, 0x28FF]];
    let expect_r: Graph      = vec![vec![0x2819, 0x28BF, 0x28FF, 0x28FF], vec![0x2800, 0x2800, 0x2819, 0x28BF]];
    assert!( horizontal_graph(false, input.clone()).unwrap().eq(&expect)   );
    assert!( horizontal_graph(true,  input.clone()).unwrap().eq(&expect_r) );
}

#[test]
fn horizontal_graph_decrementing_eight_to_one() {
    let input:    Vec<usize> = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let expect:   Graph      = vec![vec![0x28F7, 0x28C4, 0x2800, 0x2800], vec![0x28FF, 0x28FF, 0x28F7, 0x28C4]];
    let expect_r: Graph      = vec![vec![0x28FF, 0x28FF, 0x287F, 0x280B], vec![0x287F, 0x280B, 0x2800, 0x2800]];
    assert!( horizontal_graph(false, input.clone()).unwrap().eq(&expect)   );
    assert!( horizontal_graph(true,  input.clone()).unwrap().eq(&expect_r) );
}
