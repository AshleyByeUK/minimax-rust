use minimax::Node;

fn main() {
    let root = create_minimax_tree();
    println!("The score is: {}", root.score());
}

fn create_minimax_tree() -> Node {
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

    Node::new(Some(vec![depth1a, depth1b]), None)
}
