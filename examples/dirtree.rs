extern crate ptree;

use ptree::{TreeItem, PrintConfig};

use std::env;
use std::path::PathBuf;
use std::borrow::Cow;
use std::{fs, io};

#[derive(Clone, Debug)]
pub struct PathItem(pub PathBuf);

impl TreeItem for PathItem {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, config: &PrintConfig) -> io::Result<()> {
        if let Some(n) = self.0.file_name() {
            write!(f, "{}", config.paint_leaf(n.to_string_lossy()))
        } else {
            Ok(())
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let v = if let Ok(list) = fs::read_dir(&self.0) {
            list.filter_map(|item| item.ok())
                .map(|entry| entry.path())
                .map(PathItem)
                .collect()
        } else {
            Vec::new()
        };

        Cow::from(v)
    }
}


fn main() {
    let dir = PathItem(env::current_dir().expect("Unable to get current directory"));
    ptree::print_tree(&dir).expect("Unable to print directory tree");
}
