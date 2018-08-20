extern crate ptree;

use ptree::path::PathItem;

use std::env;

fn main() {
    let dir = PathItem(env::current_dir().expect("Unable to get current directory"));
    ptree::print_tree(&dir).expect("Unable to print directory tree");
}
