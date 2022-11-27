pub enum DumbTree {
    Empty,
    Node(Box<DumbTree>, Box<DumbTree>),
}

pub enum BinaryTree {
    Empty,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}

pub fn get(seek_value: i32, tree: &BinaryTree) -> Option<&BinaryTree> {
    match tree {
        BinaryTree::Node(value, left, right) => match seek_value.cmp(&value) {
            std::cmp::Ordering::Equal => Some(tree),
            std::cmp::Ordering::Less => get(seek_value, &*left),
            std::cmp::Ordering::Greater => get(seek_value, &*right),
        },
        BinaryTree::Empty => None,
    }
}

#[cfg(test)]
mod get_tests {
    use super::*;

    fn build_tree_helper() -> crate::BinaryTree {
        BinaryTree::Node(
            50,
            Box::new(BinaryTree::Node(
                25,
                Box::new(BinaryTree::Node(
                    12,
                    Box::new(BinaryTree::Empty),
                    Box::new(BinaryTree::Empty),
                )),
                Box::new(BinaryTree::Node(
                    37,
                    Box::new(BinaryTree::Empty),
                    Box::new(BinaryTree::Empty),
                )),
            )),
            Box::new(BinaryTree::Node(
                75,
                Box::new(BinaryTree::Node(
                    63,
                    Box::new(BinaryTree::Empty),
                    Box::new(BinaryTree::Empty),
                )),
                Box::new(BinaryTree::Node(
                    100,
                    Box::new(BinaryTree::Empty),
                    Box::new(BinaryTree::Empty),
                )),
            )),
        )
    }

    #[test]
    fn given_element_present() {
        let seek = 37;
        let tree = build_tree_helper();
        let result = get(seek, &tree);
        assert_eq!(result.is_some(), true);
        if let BinaryTree::Node(result_value, _, _) = result.unwrap() {
            assert_eq!(result_value.clone(), seek);
        } else {
            assert_eq!(1, -1); // Test is considered as failed if get function returns some result and its not equal to seek.
        }
    }

    #[test]
    fn given_element_is_absent() {
        let seek = 100500;
        let tree = build_tree_helper();
        let result = get(seek, &tree);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn given_tree_is_empty() {
        let seek = 37;
        let tree = BinaryTree::Empty;
        let result = get(seek, &tree);
        assert_eq!(result.is_none(), true);
    }
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

    fn build_tree_helper() -> crate::DumbTree {
        DumbTree::Node(
            Box::new(DumbTree::Node(
                Box::new(DumbTree::Node(
                    Box::new(DumbTree::Empty),
                    Box::new(DumbTree::Empty),
                )),
                Box::new(DumbTree::Node(
                    Box::new(DumbTree::Empty),
                    Box::new(DumbTree::Empty),
                )),
            )),
            Box::new(DumbTree::Node(
                Box::new(DumbTree::Node(
                    Box::new(DumbTree::Empty),
                    Box::new(DumbTree::Empty),
                )),
                Box::new(DumbTree::Node(
                    Box::new(DumbTree::Empty),
                    Box::new(DumbTree::Empty),
                )),
            )),
        )
    }

    #[test]
    fn given_tree_of_seven() {
        let tree_of_seven = build_tree_helper();
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
