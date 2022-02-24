use std::collections::VecDeque;

pub struct BPlusTree {
    root: Option<Box<Node>>,
}

struct Node {
    numbers_of_keys: usize,    // 2t ^ h - 1.
    keys: Vec<u32>,            // At least t - 1 keys, at most 2t - 1 keys
    childrens: Vec<Box<Node>>, // At least t children, at most 2t children
    is_leaf: bool,
}

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            numbers_of_keys: 0,
            keys: vec![],
            childrens: Vec::new(),
            is_leaf,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ keys: {:?}, number_of_keys: {} }}",
            self.keys, self.numbers_of_keys
        )
    }
}

impl BPlusTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: u32) {}

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        None
    }

    pub fn get(&self, key: &u32) -> Option<&u32> {
        None
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
    #[ignore]
    fn basics() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);

        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&7), Some(&7));
        assert_eq!(tree.get(&8), Some(&8));
        assert_eq!(tree.get(&9), Some(&9));
        assert_eq!(tree.get(&5), Some(&5));
        assert_eq!(tree.get(&10), Some(&10));
        assert_eq!(tree.get(&4), Some(&4));
        assert_eq!(tree.get(&12), None);
        assert_eq!(tree.get(&5), Some(&5));

        tree.remove(&7);
        // Previously, childrens linkage broke here.
        tree.remove(&16);
        tree.remove(&1);
        tree.remove(&18);

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
        let mut tree = BPlusTree::new();
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);
        tree.insert(1);
        tree.insert(2);
        tree.insert(15);
        tree.insert(13);
        tree.insert(12);
        tree.insert(26);
        tree.insert(27);
        tree.insert(28);
        tree.insert(29);

        tree.print();
        assert_eq!(tree.remove(&16), Some(16));
        tree.print();
    }

    #[test]
    #[ignore]
    fn merge_child_before_swapping_right_child_smallest_value() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);

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
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);
        tree.insert(31);
        tree.insert(32);
        tree.insert(33);
        tree.insert(34);
        tree.insert(35);

        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    #[ignore]
    fn case_3b() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);
        tree.insert(31);
        tree.insert(32);
        tree.insert(33);
        tree.insert(34);
        tree.insert(35);

        assert_eq!(tree.remove(&18), Some(18));
    }

    #[test]
    #[ignore]
    fn delete_key_on_root_node_with_internal_nodes_case_a() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);

        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    #[ignore]
    fn delete_key_on_root_node_with_internal_nodes_case_b() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);

        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    #[ignore]
    fn delete_key_on_root_node() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.remove(&8), Some(8));
        assert_eq!(tree.remove(&1), None);
        assert_eq!(tree.remove(&8), None);
    }

    #[test]
    #[ignore]
    fn delete_leaf_on_two_leaf_node() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);

        assert_eq!(tree.remove(&4), Some(4));
        assert_eq!(tree.remove(&9), Some(9));
        assert_eq!(tree.remove(&5), None);
    }

    #[test]
    #[ignore]
    fn delete_key_on_internal_node_case_a() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);

        // Actual case a
        assert_eq!(tree.remove(&4), Some(4));
    }

    #[test]
    #[ignore]
    fn delete_key_on_internal_node_case_b() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);

        assert_eq!(tree.remove(&2), Some(2));

        // Actual case b
        assert_eq!(tree.remove(&4), Some(4));
    }

    #[test]
    #[ignore]
    fn delete_key_on_internal_node_case_c() {
        let mut tree = BPlusTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);

        assert_eq!(tree.remove(&2), Some(2));
        assert_eq!(tree.remove(&5), Some(5));

        // Actual case c
        assert_eq!(tree.remove(&4), Some(4));
    }
}
