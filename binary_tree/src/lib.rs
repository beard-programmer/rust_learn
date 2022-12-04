pub enum DumbTree {
    Empty,
    Node(Box<DumbTree>, Box<DumbTree>),
}
pub enum BinaryTree {
    Empty,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}

trait NodesCountable {
    fn count_nodes(&self) -> u32;
}
impl NodesCountable for DumbTree {
    fn count_nodes(&self) -> u32 {
        fn count(tree: &DumbTree) -> u32 {
            match tree {
                DumbTree::Node(left, right) => 1 + count(&*left) + count(&*right),
                DumbTree::Empty => 0,
            }
        }

        count(self)
    }
}

trait Gettable<T, A> {
    fn get(&self, seek: T) -> Option<&A>;
}

impl Gettable<i32, BinaryTree> for BinaryTree {
    fn get(&self, seek: i32) -> Option<&BinaryTree> {
        fn find(seek: i32, tree: &BinaryTree) -> Option<&BinaryTree> {
            match tree {
                BinaryTree::Node(value, left, right) => match seek.cmp(&value) {
                    std::cmp::Ordering::Equal => Some(tree),
                    std::cmp::Ordering::Less => find(seek, &*left),
                    std::cmp::Ordering::Greater => find(seek, &*right),
                },
                BinaryTree::Empty => None,
            }
        }

        find(seek, self)
    }
}

impl NodesCountable for BinaryTree {
    fn count_nodes(&self) -> u32 {
        fn count(tree: &BinaryTree) -> u32 {
            match tree {
                BinaryTree::Node(_, left, right) => 1 + count(&*left) + count(&*right),
                BinaryTree::Empty => 0,
            }
        }

        count(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dumb_tree_tests {
        use super::*;

        mod nodes_countable_tests {
            use super::*;
            #[test]
            fn given_tree_of_seven() {
                let tree_of_seven = helpers::build_dumb_tree();
                let result = tree_of_seven.count_nodes();
                assert_eq!(result, 7);
            }

            #[test]
            fn given_empty_tree() {
                let empty_tree = DumbTree::Empty;
                let result = empty_tree.count_nodes();
                assert_eq!(result, 0);
            }
        }
    }

    mod binary_tree_tests {
        use super::*;

        mod gettable_tests {
            use super::*;
            #[test]
            fn given_element_present() {
                let seek = 37;
                let tree = helpers::build_binary_tree();
                let result = tree.get(seek);
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
                let tree = helpers::build_binary_tree();
                let result = tree.get(seek);
                assert_eq!(result.is_none(), true);
            }

            #[test]
            fn given_tree_is_empty() {
                let seek = 37;
                let tree = BinaryTree::Empty;
                let result = tree.get(seek);
                assert_eq!(result.is_none(), true);
            }
        }

        mod nodes_countable_tests {
            use super::*;
            #[test]
            fn given_tree_of_seven() {
                let tree_of_seven = helpers::build_binary_tree();
                let result = tree_of_seven.count_nodes();
                assert_eq!(result, 7);
            }

            #[test]
            fn given_empty_tree() {
                let empty_tree = BinaryTree::Empty;
                let result = empty_tree.count_nodes();
                assert_eq!(result, 0);
            }
        }
    }

    mod helpers {
        use super::*;

        pub fn build_dumb_tree() -> crate::DumbTree {
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

        pub fn build_binary_tree() -> crate::BinaryTree {
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
    }
}
