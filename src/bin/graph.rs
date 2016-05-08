// Copyright (c) 2016 Patrick Burroughs <celti@celti.name>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0>, or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

extern crate unicode_graph;

use std::{env, num, process};
use unicode_graph::{graph, braille};

#[derive(Debug)]
enum CommandError {
    UnknownError,
    ParseArgs(String),
    ParseGraph(unicode_graph::ParseGraphError),
    ParseInt(num::ParseIntError),
}

impl From<num::ParseIntError> for CommandError {
    fn from(e: num::ParseIntError) -> CommandError {
        CommandError::ParseInt(e)
    }
}

impl From<unicode_graph::ParseGraphError> for CommandError {
    fn from(e: unicode_graph::ParseGraphError) -> CommandError {
        CommandError::ParseGraph(e)
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn usage(argc: String) {
    print!("graph (from unicode_graph) v{}\n\
            Copyright © 2016 Patrick Burroughs <celti@celti.name>\n\
            \n\
            Prints a graph made of Unicode glyphs (at current only Braille patterns).
            \n\
            Usage: {} [h|v][r] <index> [<index> ...]\n\
            \x20\x20‘h’ and ‘v’ request a horizontal or vertical graph (horizontal if omitted).\n\
            \x20\x20`r` reverses the usual direction of the graph (i.e., from the top or left).\n\
            \x20\x20The remaining integer arguments indicate the size of the bar at that index.\n",
            VERSION, argc)
}

fn _main(mut argv: Vec<String>) -> Result<bool, CommandError> {
    let arg1 = match argv.get(0) {
        Some(x) => { x.clone() }
        None    => { return Ok(false) }
    };

    let (call, reverse) = match arg1.as_ref() {
        "v" => { argv.remove(0); ("vertical", false) }
        "h" => { argv.remove(0); ("horizontal", false) }
        "vr" => { argv.remove(0); ("vertical", true) }
        "hr" => { argv.remove(0); ("horizontal", true) }
        "r" => { ("horizontal", true) }
        num if num.parse::<usize>().is_ok() => { ("horizontal", false) }
        err => { return Err(CommandError::ParseArgs(err.to_owned())) }
    };

    let arg2 = match argv.get(0) {
        Some(x) => { x.clone() }
        None    => { return Err(CommandError::ParseArgs("no input specified".to_owned())) }
    };

    let reverse = match arg2.as_ref() {
        "r" => { argv.remove(0); true }
        num if num.parse::<usize>().is_ok() => { reverse }
        err => { return Err(CommandError::ParseArgs(err.to_owned())) }
    };

    let input: Vec<usize> = try!(argv.into_iter().map(|i| i.trim().parse()).collect());

    let graph = match call {
        "horizontal" => { try!(braille::horizontal_graph(reverse, input)) }
        "vertical"   => { try!(braille::vertical_graph(reverse, input)) }
        _            => { return Err(CommandError::UnknownError) }
    };

    for line in try!(graph::graph_to_strings(graph)) {
        println!("{}", line)
    };

    Ok(true)
}

fn main() {
    let mut argv: Vec<String> = env::args().collect();
    let argc: String = argv.remove(0);

    match _main(argv) {
        Ok(false)                         => { usage(argc);
                                               process::exit(0)  }
        Err(CommandError::ParseArgs(e))   => { println!("Invalid argument: {}", e);
                                               usage(argc);
                                               process::exit(64) }
        Err(CommandError::ParseInt(_))    => { println!("An error occurred parsing the input.");
                                               usage(argc);
                                               process::exit(65) }
        Err(CommandError::ParseGraph(_))  => { println!("An error occurred parsing the graph.");
                                               process::exit(65) }
        Err(CommandError::UnknownError)   => { println!("An unknown error occurred. Please report this.");
                                               process::exit(-1) }
        Ok(true)                          => { process::exit(0)  }
    }
}
