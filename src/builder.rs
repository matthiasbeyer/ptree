use item::StringItem;

///
/// A builder for a tree of [`StringItem`]s
///
/// [`StringItem`]: ../item/struct.StringItem.html
pub struct TreeBuilder {
    item: StringItem,
    level: u32,
}

impl TreeBuilder {
    ///
    /// Start building a tree
    ///
    /// The `text` argument will be the top level item's text.
    ///
    pub fn new(text: String) -> TreeBuilder {
        TreeBuilder {
            item: StringItem {
                text,
                children: Vec::new(),
            },
            level: 0,
        }
    }

    fn append_child_level(parent: &mut StringItem, level: u32, item: StringItem) {
        if level == 0 {
            parent.children.push(item);
        } else {
            TreeBuilder::append_child_level(parent.children.last_mut().unwrap(), level - 1, item);
        }
    }

    ///
    /// Add a child to the current item and make the new child current
    ///
    pub fn begin_child(&mut self, text: String) -> &mut Self {
        TreeBuilder::append_child_level(
            &mut self.item,
            self.level,
            StringItem {
                text,
                children: Vec::new(),
            },
        );
        self.level += 1;
        self
    }

    ///
    /// Finish adding children, and make the current item's parent current
    ///
    pub fn end_child(&mut self) -> &mut Self {
        self.level -= 1;
        self
    }

    ///
    /// Add an empty child (leaf item) to the current item
    ///
    pub fn add_empty_child(&mut self, text: String) -> &mut Self {
        self.begin_child(text).end_child()
    }

    ///
    /// Finish building the tree and return the top level item
    ///
    pub fn build(&mut self) -> StringItem {
        self.item.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let tree = TreeBuilder::new("test".to_string()).build();
        assert_eq!(&tree.text, "test");
        assert_eq!(tree.children.len(), 0);
    }

    #[test]
    fn single_child() {
        let tree = TreeBuilder::new("test".to_string())
            .add_empty_child("test_two".to_string())
            .build();

        assert_eq!(&tree.text, "test");
        assert_eq!(tree.children.len(), 1);
        assert_eq!(&tree.children[0].text, "test_two");
    }

    #[test]
    fn many_children_flat() {
        let mut builder = TreeBuilder::new("test".to_string());
        let n = 10;
        for i in 0..n {
            builder.add_empty_child(format!("test {}", i));
        }
        let tree = builder.build();

        assert_eq!(&tree.text, "test");
        assert_eq!(tree.children.len(), n);
        for i in 0..n {
            assert_eq!(tree.children[i].text, format!("test {}", i));
        }
    }

    #[test]
    fn many_children_nested() {
        let mut builder = TreeBuilder::new("test".to_string());
        let n = 10;
        for i in 0..n {
            builder.begin_child(format!("test {}", i));
        }
        for _ in 0..n {
            builder.end_child();
        }
        let tree = builder.build();

        assert_eq!(&tree.text, "test");

        let mut item = tree;
        for _ in 0..n {
            assert_eq!(item.children.len(), 1);
            item = item.children[0].clone();
        }

        assert_eq!(item.children.len(), 0);
    }
}
