pub enum DumbTree {
    Empty,
    Node(Box<DumbTree>, Box<DumbTree>),
}

pub fn count_nodes(tree: DumbTree) -> u32 {
    match tree {
        DumbTree::Node(left, right) => 1 + count_nodes(*left) + count_nodes(*right),
        DumbTree::Empty => 0,
    }
}

#[cfg(test)]
mod count_nodes_tests {
    use super::*;

    #[test]
    fn given_tree_of_seven() {
        let tree_of_seven = DumbTree::Node(
            Box::new(
                DumbTree::Node(
                    Box::new(DumbTree::Node(Box::new(DumbTree::Empty), Box::new(DumbTree::Empty))),
                    Box::new(DumbTree::Node(Box::new(DumbTree::Empty), Box::new(DumbTree::Empty)))
                )
            ),
            Box::new(
                DumbTree::Node(
                    Box::new(DumbTree::Node(Box::new(DumbTree::Empty), Box::new(DumbTree::Empty))),
                    Box::new(DumbTree::Node(Box::new(DumbTree::Empty), Box::new(DumbTree::Empty)))
                )
            )
        );
        let result = count_nodes(tree_of_seven);
        assert_eq!(result, 7);
    }

    #[test]
    fn given_empty_tree() {
        let empty_tree = DumbTree::Empty;
        let result = count_nodes(empty_tree);
        assert_eq!(result, 0);
    }
}