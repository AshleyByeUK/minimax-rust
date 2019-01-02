// Build a k-ary tree.
// 1. TDD out creating a node and adding children to a node.
// 2. Link nodes and determine score of top-most node. Linked list. Root node has score of 5. PAss into score function, has score of 5.
// Root has score of 5
#[derive(Debug, PartialEq)]
struct TreeNode {
    children: Box<Option<Vec<TreeNode>>>,
    score: Option<i32>,
}

impl TreeNode {
    fn new(children: Option<Vec<TreeNode>>, score: Option<i32>) -> TreeNode {
        TreeNode { children: Box::new(children), score }
    }

    fn score(&self) -> Option<i32> {
        match self.score {
            None => match self.children.as_ref() {
                None => None,
                Some(children) => children[0].score()
            },
            Some(score) => Some(score)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node_with_zero_child_nodes() {
        let node = TreeNode::new(None, None);

        assert_eq!(*node.children, None);
        assert_eq!(node.score(), None);
    }

    #[test]
    fn create_node_with_one_child_node() {
        let child = TreeNode::new(None, None);
        let node = TreeNode::new(Some(vec![child]), None);

        assert_eq!(*node.children, Some(vec![TreeNode::new(None, None)]));
        assert_eq!(node.score(), None);
    }

    #[test]
    fn create_node_with_multiple_child_nodes() {
        let child1 = TreeNode::new(None, None);
        let child2 = TreeNode::new(None, None);
        let node = TreeNode::new(Some(vec![child1, child2]), None);

        assert_eq!(*node.children, Some(vec![TreeNode::new(None, None), TreeNode::new(None, None)]));
        assert_eq!(node.score(), None);
    }

    #[test]
    fn scores_node_with_no_children() {
        let parent = TreeNode::new(None, Some(10));

        assert_eq!(parent.score(), Some(10));
    }

    #[test]
    fn scores_node_with_one_child() {
        let child = TreeNode::new(None, Some(10));
        let parent = TreeNode::new(Some(vec![child]), None);

        assert_eq!(parent.score(), Some(10));
    }

    #[test]
    fn scores_node_with_multiple_children() {
        let grandchild = TreeNode::new(None, Some(10));
        let child = TreeNode::new(Some(vec![grandchild]), None);
        let parent = TreeNode::new(Some(vec![child]), None);

        assert_eq!(parent.score(), Some(10));
    }
}
