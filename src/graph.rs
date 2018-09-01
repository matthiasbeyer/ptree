use item::TreeItem;
use output::{print_tree, write_tree_with};
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::str::from_utf8;
    use super::*;

    #[test]
    fn small_graph_output() {
        let mut deps = Graph::<&str, &str>::new();
        let pg = deps.add_node("petgraph");
        let fb = deps.add_node("fixedbitset");
        let qc = deps.add_node("quickcheck");
        let rand = deps.add_node("rand");
        let libc = deps.add_node("libc");
        deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);

        let config = PrintConfig {
            indent: 4,
            leaf: Style::default(),
            branch: Style::default(),
            ..PrintConfig::default()
        };

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        write_graph_with(&deps, pg, &mut cursor, &config).unwrap();

        let data = cursor.into_inner();
        let expected = "\
                        petgraph\n\
                        ├── quickcheck\n\
                        │   ├── libc\n\
                        │   └── rand\n\
                        │       └── libc\n\
                        └── fixedbitset\n\
                        ";
        assert_eq!(from_utf8(&data).unwrap(), expected);
    }
}
