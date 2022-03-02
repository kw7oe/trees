use std::collections::VecDeque;

pub struct BPlusTree {
    root: Option<Node>,
}

struct Node {
    keys: Vec<u32>,       // At least t - 1 keys, at most 2t - 1 keys
    values: Vec<u32>,     // Only in leaf node.
    childrens: Vec<Node>, // At least t children, at most 2t children
    is_leaf: bool,
}

const MAX_DEGREE: usize = 4;

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            keys: vec![],
            values: vec![],
            childrens: Vec::new(),
            is_leaf,
        }
    }

    pub fn insert_non_full(&mut self, key: u32) {
        match self.keys.binary_search(&key) {
            // Ignore if key is duplicated first
            Ok(_index) => (),
            Err(index) => {
                if self.is_leaf {
                    self.keys.insert(index, key);
                    self.values.insert(index, key);
                } else {
                    self.childrens[index].insert_non_full(key);

                    if self.childrens[index].keys.len() == MAX_DEGREE {
                        self.split_child(index);
                    }
                }
            }
        }
    }

    pub fn split_child(&mut self, index: usize) {
        if let Some(child) = self.childrens.get_mut(index) {
            let mut right_node = Node::new(true);
            let breakpoint = (MAX_DEGREE + 1) / 2;

            self.values.push(child.values[breakpoint]);
            self.keys.push(child.keys[breakpoint]);

            for _ in 0..breakpoint {
                let value = child.values.remove(breakpoint);
                child.keys.remove(breakpoint);
                right_node.values.push(value);
                right_node.keys.push(value);
            }

            self.childrens.push(right_node);
        }
    }

    pub fn search(&self, key: &u32) -> Option<&u32> {
        // print!("searching {key} on {:?}, index: ", self);
        let index = match self.keys.binary_search(key) {
            Ok(index) => {
                if self.is_leaf {
                    index
                } else {
                    index + 1
                }
            }
            Err(index) => index,
        };

        // println!("{index}");
        if self.is_leaf {
            self.values.get(index)
        } else {
            self.childrens[index].search(key)
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ is_leaf: {}, keys: {:?}, values: {:?}, number_of_keys: {} }}",
            self.is_leaf,
            self.keys,
            self.values,
            self.keys.len()
        )
    }
}

impl BPlusTree {
    pub fn new(numbers: Vec<u32>) -> Self {
        let mut tree = Self { root: None };

        for i in numbers {
            tree.insert(i);
        }

        tree
    }

    pub fn insert(&mut self, key: u32) {
        if let Some(node) = self.root.as_mut() {
            node.insert_non_full(key);

            if node.keys.len() == MAX_DEGREE {
                let mut new_root = Node::new(false);

                let mut child = self.root.take().unwrap();
                child.is_leaf = true;
                new_root.childrens.push(child);

                new_root.split_child(0);
                self.root = Some(new_root);
            } else {
                node.insert_non_full(key);
            }
        } else {
            let mut node = Node::new(true);
            node.insert_non_full(key);
            self.root = Some(node);
        }
    }

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        None
    }

    pub fn get(&self, key: &u32) -> Option<&u32> {
        self.root.as_ref().map_or(None, |node| node.search(key))
    }

    pub fn print(&self) {
        if let Some(node) = &self.root {
            let mut queue = VecDeque::new();
            queue.push_front(node);
            let mut visited_child = 0;
            let mut num_of_childs = 1;
            let mut next_to_visit = 0;

            while let Some(node) = queue.pop_back() {
                print!(" {:?} ", node.keys);
                // println!("{:?}: {:?}", node.keys, node.childrens);
                visited_child += 1;

                for c in &node.childrens {
                    queue.push_front(c);
                    next_to_visit += 1;
                }

                if num_of_childs == visited_child {
                    println!("");
                    visited_child = 0;
                    num_of_childs = next_to_visit;
                    next_to_visit = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BPlusTree;

    #[test]
    fn get_on_empty_tree() {
        let tree = BPlusTree::new(vec![]);
        assert_eq!(tree.get(&2), None);
    }

    #[test]
    fn insert_on_root_node() {
        let mut tree = BPlusTree::new(vec![]);

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(tree.get(&1), Some(&1));
        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&3), Some(&3));
    }

    #[test]
    fn insert_and_split_on_root_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15]);
        tree.insert(8);

        assert_eq!(tree.get(&8), Some(&8));
        assert_eq!(tree.get(&18), None);
    }

    #[test]
    fn insert_on_leaf_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15, 8]);
        tree.insert(11);
        assert_eq!(tree.get(&11), Some(&11));
    }

    #[test]
    fn insert_and_split_on_leaf_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15, 8, 11]);

        tree.insert(12);
        tree.print();
        assert_eq!(tree.get(&12), Some(&12));

        assert_eq!(tree.get(&7), Some(&7));
        assert_eq!(tree.get(&8), Some(&8));
    }

    // #[test]
    // fn insert_and_split_on_level_3_leaf_node() {
    //     let mut tree = BPlusTree::new(vec![7, 10, 15, 8, 11, 12]);

    //     tree.insert(19);
    //     tree.print();
    //     assert_eq!(tree.get(&7), Some(&7));
    //     assert_eq!(tree.get(&8), Some(&8));
    // }

    #[test]
    #[ignore]
    fn basics() {
        let mut tree = BPlusTree::new(vec![
            2, 7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30,
        ]);

        assert_eq!(tree.get(&2), Some(&2));
        // assert_eq!(tree.get(&7), Some(&7));
        // assert_eq!(tree.get(&8), Some(&8));
        // assert_eq!(tree.get(&9), Some(&9));
        // assert_eq!(tree.get(&5), Some(&5));
        // assert_eq!(tree.get(&10), Some(&10));
        // assert_eq!(tree.get(&4), Some(&4));
        // assert_eq!(tree.get(&12), None);
        // assert_eq!(tree.get(&5), Some(&5));

        // tree.remove(&7);
        // Previously, childrens linkage broke here.
        // tree.remove(&16);
        // tree.remove(&1);
        // tree.remove(&18);

        // Fixed!
        // tree.remove(&24);
        // tree.print();
        // tree.remove(&23);
        // tree.print();

        // Fixed!
        // tree.remove(&6);
        // tree.remove(&19);
        // tree.print();

        // Fixed!
        // tree.remove(&14);
        // tree.print();

        // Fixed!
        tree.remove(&21);
        tree.print();
    }

    #[test]
    #[ignore]
    fn merge_child_before_swapping_left_child_bigget_value() {
        let mut tree = BPlusTree::new(vec![
            10, 11, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 1, 2, 15, 13, 12, 26, 27, 28,
            29,
        ]);

        tree.print();
        assert_eq!(tree.remove(&16), Some(16));
        tree.print();
    }

    #[test]
    #[ignore]
    fn merge_child_before_swapping_right_child_smallest_value() {
        let mut tree = BPlusTree::new(vec![
            2, 7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30,
        ]);

        tree.print();
        assert_eq!(tree.remove(&18), Some(18));
        tree.print();

        // Steal from right, merge, and remove
        assert_eq!(tree.remove(&16), Some(16));
        tree.print();
    }

    #[test]
    #[ignore]
    fn case_3a() {
        let mut tree = BPlusTree::new(vec![
            2, 7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31,
            32, 33, 34, 35,
        ]);
        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    #[ignore]
    fn case_3b() {
        let mut tree = BPlusTree::new(vec![
            2, 7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 31,
            32, 33, 34, 35,
        ]);

        assert_eq!(tree.remove(&18), Some(18));
    }

    #[test]
    #[ignore]
    fn delete_key_on_root_node_with_internal_nodes_case_a() {
        let mut tree = BPlusTree::new(vec![2, 7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14]);
        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    #[ignore]
    fn delete_key_on_root_node_with_internal_nodes_case_b() {
        let mut tree = BPlusTree::new(vec![2, 7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14, 16, 17]);
        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    #[ignore]
    fn delete_key_on_root_node() {
        let mut tree = BPlusTree::new(vec![2, 7, 8]);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.remove(&8), Some(8));
        assert_eq!(tree.remove(&1), None);
        assert_eq!(tree.remove(&8), None);
    }

    #[test]
    #[ignore]
    fn delete_leaf_on_two_leaf_node() {
        let mut tree = BPlusTree::new(vec![2, 7, 8, 9, 4]);
        assert_eq!(tree.remove(&4), Some(4));
        assert_eq!(tree.remove(&9), Some(9));
        assert_eq!(tree.remove(&5), None);
    }

    #[test]
    #[ignore]
    fn delete_key_on_internal_node_case_a() {
        let mut tree = BPlusTree::new(vec![2, 7, 8, 9, 4, 6, 1]);

        // Actual case a
        assert_eq!(tree.remove(&4), Some(4));
    }

    #[test]
    #[ignore]
    fn delete_key_on_internal_node_case_b() {
        let mut tree = BPlusTree::new(vec![2, 7, 8, 9, 4, 6, 1, 5]);

        assert_eq!(tree.remove(&2), Some(2));

        // Actual case b
        assert_eq!(tree.remove(&4), Some(4));
    }

    #[test]
    #[ignore]
    fn delete_key_on_internal_node_case_c() {
        let mut tree = BPlusTree::new(vec![2, 7, 8, 9, 4, 6, 1, 5]);
        assert_eq!(tree.remove(&2), Some(2));
        assert_eq!(tree.remove(&5), Some(5));

        // Actual case c
        assert_eq!(tree.remove(&4), Some(4));
    }
}
