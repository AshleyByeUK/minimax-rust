// Build a k-ary tree.
// 1. TDD out creating a node and adding children to a node.
#[derive(Debug, PartialEq)]
struct TreeNode {
    children: Box<Option<Vec<TreeNode>>>,
    score: Option<i32>,
}

impl TreeNode {
    fn new(children: Option<Vec<TreeNode>>, score: Option<i32>) -> TreeNode {
        TreeNode { children: Box::new(children), score }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node_with_zero_child_nodes_and_no_score() {
        let node = TreeNode::new(None, None);

        assert_eq!(*node.children, None);
        assert_eq!(node.score, None);
    }

    #[test]
    fn create_node_with_zero_child_nodes_and_a_score() {
        let node = TreeNode::new(None, Some(1));

        assert_eq!(*node.children, None);
        assert_eq!(node.score, Some(1));
    }

    #[test]
    fn create_node_with_one_child_node() {
        let child = TreeNode::new(None, None);
        let node = TreeNode::new(Some(vec![child]), None);

        assert_eq!(*node.children, Some(vec![TreeNode::new(None, None)]));
        assert_eq!(node.score, None);
    }

    #[test]
    fn create_node_with_multiple_child_nodes() {
        let child1 = TreeNode::new(None, None);
        let child2 = TreeNode::new(None, None);
        let node = TreeNode::new(Some(vec![child1, child2]), None);

        assert_eq!(*node.children, Some(vec![TreeNode::new(None, None), TreeNode::new(None, None)]));
        assert_eq!(node.score, None);
    }
}
