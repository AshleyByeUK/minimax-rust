#[derive(Debug, PartialEq)]
struct TreeNode {
    children: Box<Option<Vec<TreeNode>>>,
    score: Option<i32>,
}

impl TreeNode {
    pub fn new(children: Option<Vec<TreeNode>>, score: Option<i32>) -> TreeNode {
        TreeNode { children: Box::new(children), score }
    }

    pub fn score(&self) -> i32 {
        self.score_depth(0)
    }

    fn score_depth(&self, depth: u32) -> i32 {
        match self.score {
            None => match self.children.as_ref() {
                None => 0,
                Some(children) => {
                    if depth % 2 == 0 {
                        children.iter()
                            .map(|child| TreeNode::score_depth(child, depth + 1))
                            .max()
                            .unwrap()
                    } else {
                        children.iter()
                            .map(|child| TreeNode::score_depth(child, depth + 1))
                            .min()
                            .unwrap()
                    }
                }
            },
            Some(score) => score
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
        assert_eq!(node.score(), 0);
    }

    #[test]
    fn create_node_with_one_child_node() {
        let child = TreeNode::new(None, None);
        let node = TreeNode::new(Some(vec![child]), None);

        assert_eq!(*node.children, Some(vec![TreeNode::new(None, None)]));
        assert_eq!(node.score(), 0);
    }

    #[test]
    fn create_node_with_multiple_child_nodes() {
        let child1 = TreeNode::new(None, None);
        let child2 = TreeNode::new(None, None);
        let node = TreeNode::new(Some(vec![child1, child2]), None);

        assert_eq!(*node.children, Some(vec![TreeNode::new(None, None), TreeNode::new(None, None)]));
        assert_eq!(node.score(), 0);
    }

    #[test]
    fn scores_node_with_no_children() {
        let parent = TreeNode::new(None, Some(10));

        assert_eq!(parent.score(), 10);
    }

    #[test]
    fn scores_node_with_one_child() {
        let child = TreeNode::new(None, Some(10));
        let parent = TreeNode::new(Some(vec![child]), None);

        assert_eq!(parent.score(), 10);
    }

    #[test]
    fn scores_node_with_multiple_children() {
        let grandchild = TreeNode::new(None, Some(10));
        let child = TreeNode::new(Some(vec![grandchild]), None);
        let parent = TreeNode::new(Some(vec![child]), None);

        assert_eq!(parent.score(), 10);
    }

    #[test]
    fn compute_score_after_two_moves_with_two_possible_end_states() {
        let grandchild1 = TreeNode::new(None, Some(1));
        let grandchild2 = TreeNode::new(None, Some(0));
        let child = TreeNode::new(Some(vec![grandchild1, grandchild2]), None);
        let parent = TreeNode::new(Some(vec![child]), None);

        assert_eq!(parent.score(), 0);
    }

    #[test]
    fn compute_score_after_two_moves_with_three_possible_end_states() {
        let grandchild1 = TreeNode::new(None, Some(12));
        let grandchild2 = TreeNode::new(None, Some(-10));
        let grandchild3 = TreeNode::new(None, Some(4));
        let child = TreeNode::new(Some(vec![grandchild1, grandchild2, grandchild3]), None);
        let parent = TreeNode::new(Some(vec![child]), None);

        assert_eq!(parent.score(), -10);
    }

    #[test]
    fn compute_score_for_game_after_four_moves_and_only_minimiser_gets_to_move() {
        let leaf1 = TreeNode::new(None, Some(1));
        let leaf2 = TreeNode::new(None, Some(0));
        let leaf3 = TreeNode::new(None, Some(2));
        let leaf4 = TreeNode::new(None, Some(-5));
        let great_grandchild1 = TreeNode::new(Some(vec![leaf1, leaf2]), None);
        let great_grandchild2 = TreeNode::new(Some(vec![leaf3, leaf4]), None);
        let grandchild1 = TreeNode::new(Some(vec![great_grandchild1]), None);
        let grandchild2 = TreeNode::new(Some(vec![great_grandchild2]), None);
        let child = TreeNode::new(Some(vec![grandchild1, grandchild2]), None);
        let parent = TreeNode::new(Some(vec![child]), None);

        assert_eq!(parent.score(), -5);
    }

    #[test]
    fn compute_score_after_one_move_with_two_possible_end_states() {
        let child1 = TreeNode::new(None, Some(0));
        let child2 = TreeNode::new(None, Some(1));
        let parent = TreeNode::new(Some(vec![child1, child2]), None);

        assert_eq!(parent.score(), 1);
    }

    #[test]
    fn compute_score_for_game_after_three_moves_and_only_maximiser_gets_to_move() {
        let leaf1 = TreeNode::new(None, Some(11));
        let leaf2 = TreeNode::new(None, Some(10));
        let leaf3 = TreeNode::new(None, Some(22));
        let leaf4 = TreeNode::new(None, Some(-5000));
        let grandchild1 = TreeNode::new(Some(vec![leaf1, leaf2]), None);
        let grandchild2 = TreeNode::new(Some(vec![leaf3, leaf4]), None);
        let child1 = TreeNode::new(Some(vec![grandchild1]), None);
        let child2 = TreeNode::new(Some(vec![grandchild2]), None);
        let parent = TreeNode::new(Some(vec![child1, child2]), None);

        assert_eq!(parent.score(), 22);
    }

    #[test]
    fn compute_score_for_deeply_nested_game() {
        let leaf1 = TreeNode::new(None, Some(1));
        let leaf2 = TreeNode::new(None, Some(-2));
        let leaf3 = TreeNode::new(None, Some(2));
        let leaf4 = TreeNode::new(None, Some(-5));
        let leaf5 = TreeNode::new(None, Some(-11));
        let leaf6 = TreeNode::new(None, Some(-30));
        let leaf7 = TreeNode::new(None, Some(20));
        let leaf8 = TreeNode::new(None, Some(51));
        let leaf9 = TreeNode::new(None, Some(12));
        let leaf10 = TreeNode::new(None, Some(10));
        let leaf11 = TreeNode::new(None, Some(6));
        let leaf12 = TreeNode::new(None, Some(7));
        let leaf13 = TreeNode::new(None, Some(8));
        let leaf14 = TreeNode::new(None, Some(9));
        let leaf15 = TreeNode::new(None, Some(3));
        let leaf16 = TreeNode::new(None, Some(4));
        let leaf17 = TreeNode::new(None, Some(7));
        let leaf18 = TreeNode::new(None, Some(5));
        let leaf19 = TreeNode::new(None, Some(9));
        let leaf20 = TreeNode::new(None, Some(11));

        let depth4a = TreeNode::new(Some(vec![leaf1, leaf2]), None);
        let depth4b = TreeNode::new(Some(vec![leaf3, leaf4]), None);
        let depth4c = TreeNode::new(Some(vec![leaf5, leaf6]), None);
        let depth4d = TreeNode::new(Some(vec![leaf7, leaf8]), None);
        let depth4e = TreeNode::new(Some(vec![leaf9, leaf10]), None);
        let depth4f = TreeNode::new(Some(vec![leaf11, leaf12]), None);
        let depth4g = TreeNode::new(Some(vec![leaf13, leaf14]), None);
        let depth4h = TreeNode::new(Some(vec![leaf15, leaf16]), None);
        let depth4i = TreeNode::new(Some(vec![leaf17, leaf18]), None);
        let depth4j = TreeNode::new(Some(vec![leaf19, leaf20]), None);

        let depth3a = TreeNode::new(Some(vec![depth4a, depth4b]), None);
        let depth3b = TreeNode::new(Some(vec![depth4c, depth4d]), None);
        let depth3c = TreeNode::new(Some(vec![depth4e, depth4f]), None);
        let depth3d = TreeNode::new(Some(vec![depth4g, depth4h]), None);
        let depth3e = TreeNode::new(Some(vec![depth4i, depth4j]), None);
        let depth3f = TreeNode::new(None, Some(1));
        let depth3g = TreeNode::new(None, Some(2));
        let depth3h = TreeNode::new(None, Some(61));

        let depth2a = TreeNode::new(Some(vec![depth3a, depth3b]), None);
        let depth2b = TreeNode::new(Some(vec![depth3c, depth3d]), None);
        let depth2c = TreeNode::new(Some(vec![depth3e, depth3f]), None);
        let depth2d = TreeNode::new(Some(vec![depth3g, depth3h]), None);

        let depth1a = TreeNode::new(Some(vec![depth2a, depth2b]), None);
        let depth1b = TreeNode::new(Some(vec![depth2c, depth2d]), None);

        let root = TreeNode::new(Some(vec![depth1a, depth1b]), None);

        assert_eq!(root.score(), 7);
    }
}
