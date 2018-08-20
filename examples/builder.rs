extern crate ansi_term;
extern crate ptree;

use ptree::{print_tree_with, PrintConfig, TreeBuilder, config::UTF_CHARS_BOLD};
use ansi_term::{Color, Style};

fn main() {
    let tree = TreeBuilder::new("house".to_string())
        .begin_child("living room".to_string())
        .add_empty_child("TV".to_string())
        .add_empty_child("couch".to_string())
        .end_child()
        .begin_child("kitchen".to_string())
        .add_empty_child("stove".to_string())
        .add_empty_child("refrigerator".to_string())
        .add_empty_child("table".to_string())
        .end_child()
        .begin_child("bathroom".to_string())
        .add_empty_child("toilet".to_string())
        .add_empty_child("shower".to_string())
        .end_child()
        .begin_child("bedroom".to_string())
        .begin_child("wardrobe".to_string())
        .add_empty_child("closet".to_string())
        .add_empty_child("shelves".to_string())
        .add_empty_child("clothes".to_string())
        .end_child()
        .add_empty_child("bed".to_string())
        .end_child()
        .build();

    let config = {
        let mut config = PrintConfig::for_stdout();
        config.branch_style = Style::new().fg(Color::Red).on(Color::Yellow).dimmed();
        config.leaf_style = Style::new().bold();
        config.chars = UTF_CHARS_BOLD;
        config.indent_size = 4;
        config
    };

    print_tree_with(&tree, &config).expect("Error printing tree");
}
