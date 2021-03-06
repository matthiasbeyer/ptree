Pretty-print tree-like structures

The `ptree` crate supports output formatting due to a user-provided configuration file and/or environment variables.
![Different output styles](https://i.imgur.com/KqPUFHq.png)

## Usage

```
[dependencies]
ptree = "0.3"
```

## Constructing a tree

There are two main ways of using `ptree` to print a tree-like data structure.
The first is to implement `TreeItem` for your structure.
The second is to create a new tree, either using `TreeBuilder` or by manually constructing `StringItem`s.

The `ptree` crate includes implementations of `TreeItem` for some common types, including a custom `StringItem` and `petgraph::Graph`.

## Printing the tree

A tree can be printed to standard output using `print_tree`, or to an arbitrary writer using `write_tree`.
Both functions have variants which take a `PrintConfig` that controls the output.
Text is formatted using `ansi-term`, which allows changing colors and styles of the printed text.

## User configuration

By default, `ptree` loads configuration from a user configuration file.
This allows end users to globally configure the output format for all applications using `ptree`.
Applications can use this configuration directly, modify it, or ignore it altogether.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
