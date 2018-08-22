use item::TreeItem;
use print_tree::{print_tree, write_tree_with};
use print_config::PrintConfig;
use style::Style;

use std::io;
use std::borrow::Cow;
use std::fmt::Display;

use petgraph::prelude::*;
use petgraph::EdgeType;
use petgraph::graph::IndexType;

impl<'a, N, E, Ty, Ix> TreeItem for (&'a Graph<N, E, Ty, Ix>, NodeIndex<Ix>)
where
    Ty: EdgeType,
    Ix: IndexType,
    N: Clone + Display,
    E: Clone,
{
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        if let Some(w) = self.0.node_weight(self.1) {
            write!(f, "{}", style.paint(w))
        } else {
            Ok(())
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let v: Vec<_> = self.0.neighbors(self.1).map(|i| (self.0, i)).collect();
        Cow::from(v)
    }
}

///
/// Print `graph`, starting at node `start`, to standard output using default formatting
///
pub fn print_graph<N, E, Ty, Ix>(graph: &Graph<N, E, Ty, Ix>, start: NodeIndex<Ix>) -> io::Result<()>
where
    Ty: EdgeType,
    Ix: IndexType,
    N: Clone + Display,
    E: Clone,
{
    print_tree(&(graph, start))
}

///
/// Write `graph`, starting at node `start`, to writer `f` using custom formatting
///
pub fn write_graph_with<N, E, Ty, Ix, W: io::Write>(
    graph: &Graph<N, E, Ty, Ix>,
    start: NodeIndex<Ix>,
    f: W,
    config: &PrintConfig,
) -> io::Result<()>
where
    Ty: EdgeType,
    Ix: IndexType,
    N: Clone + Display,
    E: Clone,
{
    write_tree_with(&(graph, start), f, config)
}
