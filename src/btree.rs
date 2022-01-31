pub struct BTree {
    root: Option<Box<Node>>,
}

struct Node {
    numbers_of_keys: u32,      // 2t ^ h - 1.
    keys: Vec<u32>,            // At least t - 1 keys, at most 2t - 1 keys
    childrens: Vec<Box<Node>>, // At least t children, at most 2t children
    is_leaf: bool,
}

const MINIMUM_DEGREE: u32 = 2; // t

impl BTree {
    pub fn new() -> BTree {
        BTree { root: None }
    }

    pub fn insert(&mut self, key: u32) {
        if let Some(node) = &mut self.root {
            if node.numbers_of_keys < 2 * MINIMUM_DEGREE - 1 {
                // TODO: Implement insert when root is not full
                println!("insert {} into root", key);
                node.numbers_of_keys += 1;
            } else {
                // TODO: Implement insert when root is full
            }
        } else {
            let node = Node {
                numbers_of_keys: 1,
                keys: vec![key],
                childrens: Vec::new(),
                is_leaf: false,
            };
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
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);

        tree.insert(5);

        assert_eq!(tree.get(&2), Some(&2));
        // assert_eq!(tree.get(&5), Some(&5));
        // assert_eq!(tree.get(&10), None);

        // tree.remove(&7);
        // assert_eq!(tree.get(&7), None);
    }
}
