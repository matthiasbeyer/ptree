extern crate ptree;

use ptree::{print_tree, TreeBuilder};

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

    print_tree(&tree).expect("Error printing tree");
}
