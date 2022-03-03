use std::collections::VecDeque;

pub struct BPlusTree {
    root: Option<Node>,
    max_degree: usize,
}

struct Node {
    keys: Vec<u32>,       // At least t - 1 keys, at most 2t - 1 keys
    values: Vec<u32>,     // Only in leaf node.
    childrens: Vec<Node>, // At least t children, at most 2t children
    is_leaf: bool,
}

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            keys: vec![],
            values: vec![],
            childrens: Vec::new(),
            is_leaf,
        }
    }

    pub fn insert_non_full(&mut self, key: u32, max_degree: usize) {
        // println!("insert_non_full key {key} into {:?}", self.keys);
        match self.keys.binary_search(&key) {
            // Ignore if key is duplicated first
            Ok(_index) => (),
            Err(index) => {
                if self.is_leaf {
                    self.keys.insert(index, key);
                    self.values.insert(index, key);
                } else {
                    self.childrens[index].insert_non_full(key, max_degree);

                    if self.childrens[index].keys.len() == max_degree {
                        self.split_child(index, max_degree);
                    }
                }
            }
        }
    }

    pub fn split_child(&mut self, index: usize, max_degree: usize) {
        // print!("parent {:?} ", self.keys);
        if let Some(child) = self.childrens.get_mut(index) {
            // println!(
            //     "splitting child at {index} with {} children: {:?}",
            //     child.childrens.len(),
            //     child.keys
            // );
            let mut right_node = Node::new(child.is_leaf);
            let breakpoint = max_degree / 2;
            let min_number_of_keys = child.keys.len() - breakpoint;

            // println!("breakpoint: {breakpoint}");

            // TODO: We probably want to rewrite the following parts
            // in a more concise a clear way.
            if index > self.keys.len() {
                if self.is_leaf {
                    self.values.push(child.values[breakpoint]);
                }
                self.keys.push(child.keys[breakpoint]);
            } else {
                // TODO: Add explanation why this is needed
                if self.is_leaf {
                    self.values.insert(index, child.values[breakpoint]);
                }
                self.keys.insert(index, child.keys[breakpoint]);
            }

            for i in 0..min_number_of_keys {
                let key = child.keys.remove(breakpoint);

                // If is leaf child, we split all the keys to the right node,
                // including the key we move to parent.
                //
                // If is internal node, there's no need to the breakpoint key
                // to the right node as it is available at the child level.
                //
                // Hence, we will skip the first key when it is not a leaf node.
                if child.is_leaf || i != 0 {
                    right_node.keys.push(key);
                }

                if child.is_leaf {
                    let value = child.values.remove(breakpoint);
                    right_node.values.push(value);
                }
            }

            // If node is leaf, means there's not children
            if !child.is_leaf {
                for _ in 0..min_number_of_keys {
                    let value = child.childrens.remove(breakpoint + 1);
                    right_node.childrens.push(value);
                }

                // Since we now have childrens, we are not leaf node anymore.
                right_node.is_leaf = false;
                right_node.values.clear();
            }

            // TODO: Add explanation why this is needed
            if index + 1 > self.childrens.len() {
                self.childrens.push(right_node);
            } else {
                self.childrens.insert(index + 1, right_node);
            }
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

    pub fn remove_from_internals(&mut self, index: usize, max_degree: usize) -> Option<u32> {
        let key = self.keys[index];

        println!("non_leaf: found key at {index}");
        self.keys.remove(index);

        if self.childrens[index + 1].keys.len() == (max_degree / 2) - 1 {
            // Case 2b
            let left_sibling = self.childrens.get_mut(index).unwrap();

            let steal_key = left_sibling.keys.pop().unwrap();
            let steal_value = left_sibling.values.pop().unwrap();

            self.keys.insert(index, steal_key);
            self.childrens[index + 1].keys.insert(0, steal_key);
            self.childrens[index + 1].values.insert(0, steal_value);

            println!("{:?}", self.childrens[index + 1]);
            self.childrens[index + 1].remove(&key, max_degree)
        } else {
            self.childrens[index + 1].remove(&key, max_degree)
        }
    }

    pub fn remove(&mut self, key: &u32, max_degree: usize) -> Option<u32> {
        println!("Removing {key} from {:?}", self);
        match self.keys.binary_search(key) {
            Ok(index) => {
                if self.is_leaf {
                    let value = self.values.remove(index);
                    self.keys.remove(index);
                    Some(value)
                } else {
                    self.remove_from_internals(index, max_degree)
                }
            }
            Err(index) => {
                if self.is_leaf {
                    None
                } else {
                    self.childrens[index].remove(key, max_degree)
                }
            }
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
    pub fn new(numbers: Vec<u32>, max_degree: usize) -> Self {
        let mut tree = Self {
            root: None,
            max_degree,
        };

        for i in numbers {
            tree.insert(i);
        }

        tree
    }

    pub fn insert(&mut self, key: u32) {
        if let Some(node) = self.root.as_mut() {
            node.insert_non_full(key, self.max_degree);

            if node.keys.len() == self.max_degree {
                let mut new_root = Node::new(false);
                new_root.childrens.push(self.root.take().unwrap());
                new_root.split_child(0, self.max_degree);
                self.root = Some(new_root);
            }
        } else {
            let mut node = Node::new(true);
            node.insert_non_full(key, self.max_degree);
            self.root = Some(node);
        }
    }

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        self.root.as_mut().map_or(None, |node| {
            let result = node.remove(key, self.max_degree);
            result
        })
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
        let tree = BPlusTree::new(vec![], 4);
        assert_eq!(tree.get(&2), None);
    }

    #[test]
    fn insert_on_root_node() {
        let mut tree = BPlusTree::new(vec![], 4);

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(tree.get(&1), Some(&1));
        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&3), Some(&3));
    }

    #[test]
    fn insert_and_split_on_root_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15], 4);
        tree.insert(8);

        assert_eq!(tree.get(&8), Some(&8));
        assert_eq!(tree.get(&18), None);
    }

    #[test]
    fn insert_on_leaf_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15, 8], 4);
        tree.insert(11);
        assert_eq!(tree.get(&11), Some(&11));
    }

    #[test]
    fn insert_and_split_on_leaf_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15, 8, 11], 4);

        tree.insert(12);
        assert_eq!(tree.get(&12), Some(&12));
        assert_eq!(tree.get(&7), Some(&7));
        assert_eq!(tree.get(&8), Some(&8));
    }

    #[test]
    fn insert_and_split_recursively_on_level_3_leaf_node() {
        let vec = vec![7, 10, 15, 8, 11, 12, 19, 25, 30];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(49);
        assert_eq!(tree.get(&49), Some(&49));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_is_reasign_to_the_right_spot() {
        let vec = vec![7, 10, 15, 8, 11, 12, 19, 25, 30, 49, 69, 90, 59];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(41);
        assert_eq!(tree.get(&41), Some(&41));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_on_existing_internal_node() {
        let vec = vec![7, 10, 15, 8, 11, 12, 19, 25, 30, 49, 69, 90, 59, 41, 45];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(42);
        assert_eq!(tree.get(&42), Some(&42));
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_on_level_4_leaf_node() {
        let vec = vec![
            7, 10, 15, 8, 11, 12, 19, 25, 30, 49, 69, 90, 59, 41, 45, 42, 1, 4, 50, 52, 5, 6, 9,
            23, 29, 26, 34,
        ];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(35);
        assert_eq!(tree.get(&35), Some(&35));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_on_level_5_leaf_node() {
        let vec: Vec<u32> = (1..82).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(82);
        assert_eq!(tree.get(&82), Some(&82));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_on_root_node() {
        let mut tree = BPlusTree::new(vec![2, 7, 8], 4);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.remove(&8), Some(8));
        assert_eq!(tree.remove(&1), None);
        assert_eq!(tree.remove(&8), None);
    }

    #[test]
    fn delete_key_case1a() {
        let mut vec = vec![2, 7, 8, 9, 4, 6, 1, 5, 3];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.get(&7), None);

        vec.remove(1);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_case1b() {
        let mut vec = vec![15, 25, 35, 5, 45, 20, 30, 55, 40];
        let mut tree = BPlusTree::new(vec.clone(), 3);

        tree.print();
        assert_eq!(tree.remove(&5), Some(5));
        tree.print();

        vec.retain(|&x| x != 5);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_case2b() {
        let mut vec = vec![2, 7, 8, 9, 4, 6, 1, 5, 3];
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&7);

        assert_eq!(tree.remove(&6), Some(6));
        assert_eq!(tree.get(&6), None);

        vec.retain(|&x| x != 7 && x != 6);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    #[ignore]
    fn delete_key_case2c() {
        let mut vec = vec![
            7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30,
        ];
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&24);

        assert_eq!(tree.remove(&23), Some(23));
        assert_eq!(tree.get(&23), None);

        vec.retain(|&x| x != 24 && x != 23);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    #[ignore]
    fn delete_key_case3() {
        let mut vec = vec![2, 7, 8, 9, 10, 12];
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&7);

        assert_eq!(tree.remove(&2), Some(2));
        assert_eq!(tree.get(&2), None);

        vec.retain(|&x| x != 7 && x != 2);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }
}
