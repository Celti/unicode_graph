# unicode_graph: Unicode glyph graphs with Rust
Inspired by the Python [graph](https://github.com/chrisbouchard/graph) library.

My first version was in [bash](https://github.com/Celti/graph.sh); now it's a Rust crate offering both a library and binary using nothing but std.

## Utility
As the bash version, but much faster.

Usage: `graph [h|v] <index> [<index> ...]`

`h` or `v` indicate whether you want a horizontal or vertical graph, respectively, defaulting to horizontal if unspecified. Further arguments are a list of column heights or row lengths. The graph grows upward or rightwards.

Returns 64 if it can't parse the command you gave it, 65 if it can't parse your input into a graph, or 255 if you somehow made it try to do something it doesn't know how to do — and, of course, 0 on a success.

### Examples
```
$ ./graph h 3 2 1 4
⣦⣸
```

```
$ ./graph v 3 2 1 4
⣟⣁
```

```
$ ./graph h $(seq 1 3 32) $(seq 31 -2 1) $(seq 12 -1 1) $(seq 1 12)
⠀⠀⠀⠀⠀⣶⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⣸⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢠⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣾⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣸⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢠⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⢸⣦⡀⠀⠀⠀⠀⠀⠀⠀⢀⣴⡇
⠀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⢸⣿⣿⣦⡀⠀⠀⠀⢀⣴⣿⣿⡇
⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣸⣿⣿⣿⣿⣦⣀⣴⣿⣿⣿⣿⡇
```

```
 $ ./graph v $(seq 12 -1 1) $(seq 30 -2 10) $(seq 0 1 24) $(seq 0 2 48)
⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠿⠿⠿⠿⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣶⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣶⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣀⠀⠀⠀⠀⠀
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣀⠀
⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
```

## Library
```
extern crate unicode_graph;
use unicode_graph::graph;
use unicode_graph::braille;
```

```
fn braille::horizontal_graph (input: Vec<usize>)    -> Result<Vec<Vec<u32>>, ParseGraphError>
fn braille::vertical_graph   (input: Vec<usize>)    -> Result<Vec<Vec<u32>>, ParseGraphError>
fn graph::graph_to_strings   (input: Vec<Vec<u32>>) -> Result<Vec<String>,   ParseGraphError>
struct graph::ParseGraphError
```

The input vector to `braille::*_graph()` (`Vec<usize>`) is a simple list of bar sizes (height or length). The output from them (`Vec<Vec<u32>`) is a list of lines of Unicode code points in the range of 0x2800 to 0x28FF (the [Braille Patterns block](http://unicode-table.com/en/blocks/braille-patterns/)).

The `graph_to_strings()` function takes one of those `Vec<Vec<u32>>`s and converts it to a list of `String`s, one per line of the graph.

The `ParseGraphError` type is a dumb error type that has no real information, mostly there just to have something to return in poorly planned situations. I don't think `*_graph()` can technically return one unless you feed it something silly like an empty or `unsafe`ly mangled vector, and `graph_to_strings()` will only return it if you feed it something that can't become a `char`.

## Plotting...
* Tests
* Reversible graphs
* Big chunky graphs that use 0x2580–0x259F ([Block Elements](http://unicode-table.com/en/blocks/block-elements/))
* Unchecked functions that just zero out anything they don't like
* Useful error type(s) that tell you where in the input they failed
* World domination

## License & Copyright
Copyright (c) 2016 by Patrick Burroughs <celti@celti.name>

Licensed under the Apache License, Version 2.0 <[LICENSE-APACHE](LICENSE-APACHE) or [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)>, or the MIT license <[LICENSE-MIT](LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)>, at your option. This file may not be copied, modified, or distributed except according to those terms.
