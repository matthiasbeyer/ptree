#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/ptree/0.3.1")]

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
//!     .begin_child("branch".to_string())
//!         .add_empty_child("leaf".to_string())
//!     .end_child()
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
//! ## Output configuration
//!
//! Ptree allows user configuration of the output format.
//! Thus any program using the library can be configured globaly,
//! providing a consistent user experience.
//!
//! Output formatting is controlled by a user configuration file or
//! by environment variables.
//!
//! ```toml
//! # <config_dir>/ptree.toml
//!
//! indent = 4
//!
//! [branch]
//! foreground = red
//! dimmed = true
//!
//! [leaf]
//! bold = true
//! ```
//!
//! The configuration file resides in the platform-specific user configuration directory,
//! as returned by [`config_dir`].
//! It can be in TOML, YAML, INI or JSON format, provided the file stem is `ptree`.
//! A custom configuration file can be specified by setting the `PTREE_CONFIG` environment
//! variable to the full path of the file.
//!
//! Individual configuration parameters can also be overriden using environment variables.
//!
//! ```bash
//! PTREE_INDENT=3 PTREE_BRANCH_BACKGROUND=yellow <command>
//! ```
//!
//! See [`PrintConfig`] for the list of all configuration options.
//!
//! ## Advanced usage
//!
//! ### Implementing the `TreeItem` trait
//!
//! Rather than construct a new tree, one can implement the
//! [`TreeItem`] trait for a custom data structure.
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
//! ### Custom output formatting
//!
//! The [`print_tree`] function loads the user configuration to control
//! output formatting.
//! If you want to override this, you can create your own PrintConfig
//! and use the [`print_tree_with`] function.
//!
//! ```
//! # use std::collections::HashMap;
//! # use std::io;
//! # use ptree::{print_tree_with, TreeBuilder, PrintConfig};
//! # use ptree::print_config::UTF_CHARS_BOLD;
//! # use ptree::{Color, Style};
//! # fn main() -> Result<(), io::Error> {
//! // Build a tree using a TreeBuilder
//! let tree = TreeBuilder::new("tree".to_string())
//!     .add_empty_child("empty branch".to_string())
//!     .build();
//!
//! // Set up the print configuration
//! let config = {
//!     let mut config = PrintConfig::from_env();
//!     config.branch = Style {
//!         foreground: Some(Color::Red),
//!         background: Some(Color::Yellow),
//!         dimmed: true,
//!         ..Style::default()
//!     };
//!     config.leaf = Style {
//!         bold: true,
//!         ..Style::default()
//!     };
//!     config.characters = UTF_CHARS_BOLD.into();
//!     config.indent = 4;
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
//! ### Write to a file
//!
//! To write a tree to a file rather than to standard output,
//! use [`write_tree`] or [`write_tree_with`].
//!
//! Unless [`PrintConfig::styled`] is set to [`Always`], these two functions
//! will not use ANSI coloring and styling for the output text.
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
//!
//! [`config_dir`]: https://docs.rs/directories/1.0.1/directories/struct.BaseDirs.html#method.config_dir
//! [`TreeItem`]: item/trait.TreeItem.html
//! [`print_tree`]: output/fn.print_tree.html
//! [`print_tree_with`]: output/fn.print_tree_with.html
//! [`write_tree`]: output/fn.write_tree.html
//! [`write_tree_with`]: output/fn.write_tree_with.html
//! [`PrintConfig::styled`]: print_config/struct.PrintConfig.html#structfield.styled
//! [`Always`]: print_config/struct.PrintConfig.html#structfield.styled
//! [`PrintConfig`]: print_config/struct.PrintConfig.html

#[cfg(feature = "petgraph")]
extern crate petgraph;

#[cfg(feature = "ansi")]
extern crate ansi_term;
#[cfg(feature = "ansi")]
extern crate atty;
#[cfg(feature = "ansi")]
extern crate tint;

#[cfg(feature = "value")]
extern crate serde_value;

extern crate config;
extern crate directories;
extern crate serde;

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
pub mod output;

#[cfg(feature = "petgraph")]
///
/// Implementation of `TreeItem` for [`petgraph::Graph`]
///
/// This module is enabled by the `"petgraph"` feature.
///
/// [`petgraph::Graph`]: https://docs.rs/petgraph/0.4.13/petgraph/graph/struct.Graph.html
pub mod graph;

#[cfg(feature = "value")]
///
/// Implementation of `TreeItem` for [`serde_value::Value`], allowing easy printing of
/// deserialized structures from a variety of formats.
///
/// This module is enabled by the `"serde"` feature.
///
/// [`TreeItem`]: item/trait.TreeItem.html
pub mod value;

pub use builder::TreeBuilder;
pub use item::TreeItem;
pub use output::{print_tree, print_tree_with, write_tree, write_tree_with};
pub use print_config::{IndentChars, PrintConfig};
pub use style::{Color, Style};

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate serde_any;
