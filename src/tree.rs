use tree_sitter::{Node, Tree};

/// Pre-order traversal of the syntax tree that triggers @node_fn at each node
pub fn traverse<CB>(tree: &Tree, mut node_fn: CB)
where
    CB: FnMut(&Node),
{
    let mut cursor = tree.walk();
    'outer: loop {
        let node = cursor.node();
        node_fn(&node);
        if !cursor.goto_first_child() && !cursor.goto_next_sibling() {
            'inner: loop {
                if !cursor.goto_parent() {
                    break 'outer;
                }
                if cursor.goto_next_sibling() {
                    break 'inner;
                }
            }
        }
    }
}
