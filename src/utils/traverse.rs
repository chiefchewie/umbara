use tree_sitter::{Node, TreeCursor};

pub fn traverse(cursor: TreeCursor) -> TreeIterator {
    TreeIterator {
        cursor: Some(cursor),
    }
}

pub struct TreeIterator<'a> {
    cursor: Option<TreeCursor<'a>>,
}

pub struct TreeItem<'a> {
    pub node: Node<'a>,
    pub field: &'a str,
}

impl TreeItem<'_> {
    fn from<'a>(node: Node<'a>, field: &'a str) -> TreeItem<'a> {
        TreeItem { node, field }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = TreeItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.cursor.as_mut() {
            Some(cursor) => cursor,
            None => return None,
        };

        let node = c.node();

        let field = match c.field_name() {
            Some(name) => name,
            None => "None",
        };

        if c.goto_first_child() || c.goto_next_sibling() {
            return Some(TreeItem::from(node, field));
        }

        loop {
            // If we can't go to the parent, then that means we've reached the root, and our
            // iterator will be done in the next iteration
            if !c.goto_parent() {
                self.cursor = None;
                break;
            }

            // If we get to a sibling, then this will be the first time we touch that node,
            // so it'll be the next starting node
            if c.goto_next_sibling() {
                break;
            }
        }

        return Some(TreeItem::from(node, field));
    }
}
