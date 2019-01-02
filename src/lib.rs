// Build a k-ary tree.
// 1. TDD out creating a node and adding children to a node.
#[derive(Debug, PartialEq)]
struct TreeNode {
    children: Box<Option<Vec<TreeNode>>>,
}

impl TreeNode {
    fn new(children: Option<Vec<TreeNode>>) -> TreeNode {
        TreeNode { children: Box::new(children) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node_with_zero_child_nodes() {
        let node = TreeNode::new(None);

        assert_eq!(node, TreeNode { children: Box::new(None) });
    }

    #[test]
    fn create_node_with_one_child_node() {
        let child = TreeNode::new(None);

        let node = TreeNode::new(Some(vec![child]));

        assert_eq!(node, TreeNode {
            children: Box::new(Some(vec![
                TreeNode { children: Box::new(None) }
            ]))
        });
    }

    #[test]
    fn create_node_with_multiple_child_nodes() {
        let child1 = TreeNode::new(None);
        let child2 = TreeNode::new(None);

        let node = TreeNode::new(Some(vec![child1, child2]));

        assert_eq!(node, TreeNode {
            children: Box::new(Some(vec![
                TreeNode { children: Box::new(None) },
                TreeNode { children: Box::new(None) },
            ]))
        });
    }
}
