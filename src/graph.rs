use item::TreeItem;
use print_tree::{print_tree, write_tree_with};
use config::PrintConfig;

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

    fn write_self<W: io::Write>(&self, f: &mut W, config: &PrintConfig) -> io::Result<()> {
        if let Some(w) = self.0.node_weight(self.1) {
            write!(f, "{}", config.paint_leaf(w.to_string()))
        } else {
            Ok(())
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let v: Vec<_> = self.0.neighbors(self.1).map(|i| (self.0, i)).collect();
        Cow::from(v)
    }
}

pub fn print_graph<N, E, Ty, Ix>(
    graph: &Graph<N, E, Ty, Ix>,
    start: NodeIndex<Ix>,
) -> io::Result<()>
where
    Ty: EdgeType,
    Ix: IndexType,
    N: Clone + Display,
    E: Clone,
{
    print_tree(&(graph, start))
}

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
