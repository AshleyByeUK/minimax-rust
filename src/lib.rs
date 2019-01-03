extern crate num;

use num::Integer;

#[derive(Debug, PartialEq)]
pub struct Node {
    children: Box<Option<Vec<Node>>>,
    score: Option<i32>,
}

impl Node {
    pub fn new(children: Option<Vec<Node>>, score: Option<i32>) -> Node {
        Node { children: Box::new(children), score }
    }

    pub fn score(&self) -> i32 {
        self.score_node(0)
    }

    fn score_node(&self, depth: u32) -> i32 {
        match self.score {
            Some(score) => score,
            None => Self::score_children(self.children.as_ref(), depth + 1)
        }
    }

    fn score_children(children: &Option<Vec<Node>>, depth: u32) -> i32 {
        match children {
            None => 0,
            Some(children) => children.iter()
                .map(|child| Self::score_node(child, depth))
                .min_or_max((depth - 1).is_even())
                .unwrap()
        }
    }
}

pub trait MinOrMax: Iterator {
    fn min_or_max(self, is_maximising: bool) -> Option<Self::Item> where Self: Sized, Self::Item: Ord;
}

impl<I> MinOrMax for I where I: Iterator {
    fn min_or_max(self, is_maximising: bool) -> Option<Self::Item> where Self: Sized, Self::Item: Ord {
        if is_maximising {
            self.max()
        } else {
            self.min()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node_with_zero_child_nodes() {
        let node = Node::new(None, None);

        assert_eq!(*node.children, None);
        assert_eq!(node.score(), 0);
    }

    #[test]
    fn create_node_with_one_child_node() {
        let child = Node::new(None, None);
        let node = Node::new(Some(vec![child]), None);

        assert_eq!(*node.children, Some(vec![Node::new(None, None)]));
        assert_eq!(node.score(), 0);
    }

    #[test]
    fn create_node_with_multiple_child_nodes() {
        let child1 = Node::new(None, None);
        let child2 = Node::new(None, None);
        let node = Node::new(Some(vec![child1, child2]), None);

        assert_eq!(*node.children, Some(vec![Node::new(None, None), Node::new(None, None)]));
        assert_eq!(node.score(), 0);
    }

    #[test]
    fn scores_node_with_no_children() {
        let parent = Node::new(None, Some(10));

        assert_eq!(parent.score(), 10);
    }

    #[test]
    fn scores_node_with_one_child() {
        let child = Node::new(None, Some(10));
        let parent = Node::new(Some(vec![child]), None);

        assert_eq!(parent.score(), 10);
    }

    #[test]
    fn scores_node_with_multiple_children() {
        let grandchild = Node::new(None, Some(10));
        let child = Node::new(Some(vec![grandchild]), None);
        let parent = Node::new(Some(vec![child]), None);

        assert_eq!(parent.score(), 10);
    }

    #[test]
    fn compute_score_after_two_moves_with_two_possible_end_states() {
        let grandchild1 = Node::new(None, Some(1));
        let grandchild2 = Node::new(None, Some(0));
        let child = Node::new(Some(vec![grandchild1, grandchild2]), None);
        let parent = Node::new(Some(vec![child]), None);

        assert_eq!(parent.score(), 0);
    }

    #[test]
    fn compute_score_after_two_moves_with_three_possible_end_states() {
        let grandchild1 = Node::new(None, Some(12));
        let grandchild2 = Node::new(None, Some(-10));
        let grandchild3 = Node::new(None, Some(4));
        let child = Node::new(Some(vec![grandchild1, grandchild2, grandchild3]), None);
        let parent = Node::new(Some(vec![child]), None);

        assert_eq!(parent.score(), -10);
    }

    #[test]
    fn compute_score_for_game_after_four_moves_and_only_minimiser_gets_to_move() {
        let leaf1 = Node::new(None, Some(1));
        let leaf2 = Node::new(None, Some(0));
        let leaf3 = Node::new(None, Some(2));
        let leaf4 = Node::new(None, Some(-5));
        let great_grandchild1 = Node::new(Some(vec![leaf1, leaf2]), None);
        let great_grandchild2 = Node::new(Some(vec![leaf3, leaf4]), None);
        let grandchild1 = Node::new(Some(vec![great_grandchild1]), None);
        let grandchild2 = Node::new(Some(vec![great_grandchild2]), None);
        let child = Node::new(Some(vec![grandchild1, grandchild2]), None);
        let parent = Node::new(Some(vec![child]), None);

        assert_eq!(parent.score(), -5);
    }

    #[test]
    fn compute_score_after_one_move_with_two_possible_end_states() {
        let child1 = Node::new(None, Some(0));
        let child2 = Node::new(None, Some(1));
        let parent = Node::new(Some(vec![child1, child2]), None);

        assert_eq!(parent.score(), 1);
    }

    #[test]
    fn compute_score_for_game_after_three_moves_and_only_maximiser_gets_to_move() {
        let leaf1 = Node::new(None, Some(11));
        let leaf2 = Node::new(None, Some(10));
        let leaf3 = Node::new(None, Some(22));
        let leaf4 = Node::new(None, Some(-5000));
        let grandchild1 = Node::new(Some(vec![leaf1, leaf2]), None);
        let grandchild2 = Node::new(Some(vec![leaf3, leaf4]), None);
        let child1 = Node::new(Some(vec![grandchild1]), None);
        let child2 = Node::new(Some(vec![grandchild2]), None);
        let parent = Node::new(Some(vec![child1, child2]), None);

        assert_eq!(parent.score(), 22);
    }

    #[test]
    fn compute_score_for_deeply_nested_game() {
        let leaf1 = Node::new(None, Some(1));
        let leaf2 = Node::new(None, Some(-2));
        let leaf3 = Node::new(None, Some(2));
        let leaf4 = Node::new(None, Some(-5));
        let leaf5 = Node::new(None, Some(-11));
        let leaf6 = Node::new(None, Some(-30));
        let leaf7 = Node::new(None, Some(20));
        let leaf8 = Node::new(None, Some(51));
        let leaf9 = Node::new(None, Some(12));
        let leaf10 = Node::new(None, Some(10));
        let leaf11 = Node::new(None, Some(6));
        let leaf12 = Node::new(None, Some(7));
        let leaf13 = Node::new(None, Some(8));
        let leaf14 = Node::new(None, Some(9));
        let leaf15 = Node::new(None, Some(3));
        let leaf16 = Node::new(None, Some(4));
        let leaf17 = Node::new(None, Some(7));
        let leaf18 = Node::new(None, Some(5));
        let leaf19 = Node::new(None, Some(9));
        let leaf20 = Node::new(None, Some(11));

        let depth4a = Node::new(Some(vec![leaf1, leaf2]), None);
        let depth4b = Node::new(Some(vec![leaf3, leaf4]), None);
        let depth4c = Node::new(Some(vec![leaf5, leaf6]), None);
        let depth4d = Node::new(Some(vec![leaf7, leaf8]), None);
        let depth4e = Node::new(Some(vec![leaf9, leaf10]), None);
        let depth4f = Node::new(Some(vec![leaf11, leaf12]), None);
        let depth4g = Node::new(Some(vec![leaf13, leaf14]), None);
        let depth4h = Node::new(Some(vec![leaf15, leaf16]), None);
        let depth4i = Node::new(Some(vec![leaf17, leaf18]), None);
        let depth4j = Node::new(Some(vec![leaf19, leaf20]), None);

        let depth3a = Node::new(Some(vec![depth4a, depth4b]), None);
        let depth3b = Node::new(Some(vec![depth4c, depth4d]), None);
        let depth3c = Node::new(Some(vec![depth4e, depth4f]), None);
        let depth3d = Node::new(Some(vec![depth4g, depth4h]), None);
        let depth3e = Node::new(Some(vec![depth4i, depth4j]), None);
        let depth3f = Node::new(None, Some(1));
        let depth3g = Node::new(None, Some(2));
        let depth3h = Node::new(None, Some(61));

        let depth2a = Node::new(Some(vec![depth3a, depth3b]), None);
        let depth2b = Node::new(Some(vec![depth3c, depth3d]), None);
        let depth2c = Node::new(Some(vec![depth3e, depth3f]), None);
        let depth2d = Node::new(Some(vec![depth3g, depth3h]), None);

        let depth1a = Node::new(Some(vec![depth2a, depth2b]), None);
        let depth1b = Node::new(Some(vec![depth2c, depth2d]), None);

        let root = Node::new(Some(vec![depth1a, depth1b]), None);

        assert_eq!(root.score(), 7);
    }
}
