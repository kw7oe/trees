pub struct BTree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    numbers_of_keys: u32,      // 2t ^ h - 1.
    keys: Vec<u32>,            // At least t - 1 keys, at most 2t - 1 keys
    childrens: Vec<Box<Node>>, // At least t children, at most 2t children
    is_leaf: bool,
}

const MINIMUM_DEGREE: u32 = 2; // t
const MAX_DEGREE: u32 = 2 * MINIMUM_DEGREE - 1;

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            numbers_of_keys: 0,
            keys: vec![],
            childrens: Vec::new(),
            is_leaf,
        }
    }

    pub fn split_child(&mut self, index: usize) {
        // Currently works for split root node only.
        if let Some(child) = self.childrens.get_mut(index) {
            let mut new_node = Self::new(child.is_leaf);
            let new_node_number_of_keys = MINIMUM_DEGREE - 1;
            new_node.numbers_of_keys = new_node_number_of_keys;

            // Split keys to new node
            for _ in 0..new_node_number_of_keys {
                if let Some(key) = child.keys.pop() {
                    new_node.keys.push(key);
                    child.numbers_of_keys -= 1;
                }
            }

            // Split childs to new node if not leaf node

            // Now assign both nodes to parents
            self.childrens.push(Box::new(new_node));
        };
    }

    pub fn insert_non_full(&mut self, key: u32) {
        match self.keys.binary_search(&key) {
            Ok(_) => (),
            Err(pos) => {
                self.keys.insert(pos, key);
            }
        }

        self.numbers_of_keys += 1;
    }
}

impl BTree {
    pub fn new() -> BTree {
        BTree { root: None }
    }

    pub fn insert(&mut self, key: u32) {
        if let Some(node) = &mut self.root {
            if node.numbers_of_keys == MAX_DEGREE {
                let mut new_root = Node::new(false);
                new_root.childrens.push(self.root.take().unwrap());
                new_root.split_child(0);
                new_root.insert_non_full(key);
                self.root = Some(Box::new(new_root));
            } else {
                node.insert_non_full(key);
            }
        } else {
            let mut node = Node::new(false);
            node.insert_non_full(key);
            self.root = Some(Box::new(node));
        }
    }

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        Some(*key)
    }

    pub fn get(&self, key: &u32) -> Option<&u32> {
        if let Some(node) = &self.root {
            node.keys
                .binary_search(key)
                .map_or(None, |index| node.keys.get(index))
        } else {
            None
        }
    }

    pub fn print(&self) {
        if let Some(node) = &self.root {
            println!("{:?}", node);
            for n in &node.childrens {
                println!("{:?}", n);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BTree;

    #[test]
    fn basics() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        // tree.insert(4);
        // tree.insert(6);
        // tree.insert(1);

        // tree.insert(5);

        tree.print();

        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&7), Some(&7));
        assert_eq!(tree.get(&8), Some(&8));
        assert_eq!(tree.get(&9), Some(&9));
        // assert_eq!(tree.get(&5), Some(&5));
        // assert_eq!(tree.get(&10), None);

        // tree.remove(&7);
        // assert_eq!(tree.get(&7), None);
    }
}
