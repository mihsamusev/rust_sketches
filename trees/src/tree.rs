pub type TreeIndex = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub value: usize,
    pub left: Option<TreeIndex>,
    pub right: Option<TreeIndex>,
}

impl TreeNode {
    pub fn new(value: usize, left: Option<TreeIndex>, right: Option<TreeIndex>) -> Self {
        TreeNode { value, left, right }
    }
}

#[derive(Debug)]
pub struct Tree {
    // stable arena, removing nodes are not popped, just converted from Some to None
    arena: Vec<Option<TreeNode>>,
    root: Option<TreeIndex>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            root: None,
        }
    }

    pub fn iter(&self) -> PreorderIter {
        PreorderIter::new(self.root)
    }

    pub fn set_root(&mut self, root: Option<TreeIndex>) {
        self.root = root
    }

    pub fn add_node(&mut self, node: TreeNode) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        return index;
    }

    pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode> {
        if let Some(node) = self.arena.get_mut(index) {
            node.take()
        } else {
            None
        }
    }

    pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        };
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        };
    }
}

pub struct PreorderIter {
    stack: Vec<TreeIndex>,
}

impl PreorderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreorderIter { stack: vec![index] }
        } else {
            PreorderIter { stack: vec![] }
        }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(node_index) = self.stack.pop() {
            if let Some(node) = tree.node_at(node_index) {
                if let Some(right) = node.right {
                    self.stack.push(right)
                }

                if let Some(left) = node.left {
                    self.stack.push(left)
                }

                return Some(node_index);
            }
        }

        return None;
    } // immutable borrow &Tree ends here
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tree() -> Tree {
        let mut tree = Tree::new();
        let a = tree.add_node(TreeNode::new(4, None, None));
        let b = tree.add_node(TreeNode::new(5, None, None));
        let c = tree.add_node(TreeNode::new(2, Some(a), Some(b)));
        let d = tree.add_node(TreeNode::new(3, None, None));
        let e = tree.add_node(TreeNode::new(1, Some(c), Some(d)));
        tree.set_root(Some(e));
        tree
    }

    #[test]
    fn given_buiild_tree_iterating_with_preorder_iter_gives_correct_values() {
        let tree = get_tree();
        let mut preorder = tree.iter();
        let mut values = vec![];
        while let Some(i) = preorder.next(&tree) {
            if let Some(n) = tree.node_at(i) {
                values.push(n.value);
            }
        }
        let expected_values: Vec<usize> = vec![1, 2, 4, 5, 3];
        assert_eq!(values, expected_values);
    }
}
