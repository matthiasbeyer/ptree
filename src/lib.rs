#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/ptree/0.1.0")]

//! # ptree
//!
//! Pretty-print tree-like structures
//!
//! ## Basic usage
//!
//! ```
//! # use std::collections::HashMap;
//! # use std::io;
//! # use ptree::{print_tree, TreeBuilder};
//! # fn main() -> Result<(), io::Error> {
//! // Build a tree using a TreeBuilder
//! let tree = TreeBuilder::new("tree".to_string())
//!     .add_empty_child("empty branch".to_string())
//!     .build();
//!
//! // Print out the tree using default formatting
//! print_tree(&tree)?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Implementing the `TreeItem` trait
//!
//! Rather than construct a new tree, one can implement the
//! `TreeItem` trait for a custom data structure.
//!
//! ```
//! # use std::collections::HashMap;
//! # use std::{io, borrow::Cow};
//! # use ptree::{print_tree, TreeItem, Style};
//! #[derive(Clone)]
//! struct MyCustomTree {}
//!
//! impl TreeItem for MyCustomTree {
//!     type Child = Self;
//!     fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
//!         write!(f, "{}", style.paint("My custom tree"))
//!     }
//!     fn children(&self) -> Cow<[Self::Child]> {
//!         Cow::from(vec![])
//!     }
//! }
//!
//! # fn main() -> Result<(), io::Error> {
//! // Build my custom tree structure
//! let tree = MyCustomTree {};
//!
//! // Print out the tree using default formatting
//! print_tree(&tree)?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Output formatting
//!
//! ```
//! # use std::collections::HashMap;
//! # use std::io;
//! # use ptree::{print_tree_with, TreeBuilder, PrintConfig};
//! # use ptree::config::UTF_CHARS_BOLD;
//! # use ptree::{Color, Style};
//! # fn main() -> Result<(), io::Error> {
//! // Build a tree using a TreeBuilder
//! let tree = TreeBuilder::new("tree".to_string())
//!     .add_empty_child("empty branch".to_string())
//!     .build();
//!
//! // Set up the print configuration
//! let config = {
//!     let mut config = PrintConfig::for_stdout();
//!     config.branch_style = Style {
//!         foreground: Some(Color::Red),
//!         background: Some(Color::Yellow),
//!         dimmed: true,
//!         ..Style::default()
//!     };
//!     config.leaf_style = Style {
//!         bold: true,
//!         ..Style::default()
//!     };
//!     config.chars = UTF_CHARS_BOLD.into();
//!     config.indent_size = 4;
//!     config
//! };
//!
//! // Print out the tree using custom formatting
//! print_tree_with(&tree, &config)?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Write to a file
//!
//! ```
//! # use std::collections::HashMap;
//! # use std::io;
//! # use std::error::Error;
//! # use std::fs::{File, remove_file};
//! # use ptree::{write_tree, TreeBuilder};
//! # fn main() -> Result<(), Box<Error>> {
//! // Build a tree using a TreeBuilder
//! let tree = TreeBuilder::new("tree".to_string())
//!     .add_empty_child("empty branch".to_string())
//!     .build();
//!
//! // Open a file for writing
//! let file_name = "tree.txt";
//! let file = File::create(&file_name)?;
//!
//! // Write out the tree to the file
//! write_tree(&tree, file)?;
//!
//! # remove_file(&file_name)?;
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "petgraph")]
extern crate petgraph;

#[cfg(feature = "ansi")]
extern crate ansi_term;
#[cfg(feature = "ansi")]
extern crate isatty;
#[cfg(feature = "ansi")]
extern crate tint;

#[cfg(feature = "value")]
extern crate serde_value;

extern crate directories;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate config;

///
/// Contains the `TreeItem` trait
///
pub mod item;

///
/// Contains the `TreeBuilder` structure, useful for manually constructing trees
///
pub mod builder;

///
/// Structures to control the output formatting
///
pub mod print_config;

///
/// Structures to control terminal colors and styles
///
pub mod style;

///
/// Functions for printing trees to standard output or to custom writers
///
pub mod print_tree;

#[cfg(feature = "petgraph")]
///
/// Implementation of `TreeItem` for `petgraph::Graph`
///
/// This module is enabled by the `"petgraph"` feature.
///
pub mod graph;

#[cfg(feature = "serde")]
///
/// Implementation of `TreeItem` for `serde_value::Value`, allowing easy printing
/// deserialized structures from a variety of formats.
///
/// This module is enabled by the `"serde"` feature.
///
pub mod value;

pub use print_tree::{print_tree, print_tree_with, write_tree, write_tree_with};
pub use builder::TreeBuilder;
pub use item::TreeItem;
pub use print_config::{IndentChars, PrintConfig};
pub use style::{Color, Style};
