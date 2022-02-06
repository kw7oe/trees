pub struct BTree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    numbers_of_keys: usize,    // 2t ^ h - 1.
    keys: Vec<u32>,            // At least t - 1 keys, at most 2t - 1 keys
    childrens: Vec<Box<Node>>, // At least t children, at most 2t children
    is_leaf: bool,
}

const MINIMUM_DEGREE: usize = 2; // t
const MAX_DEGREE: usize = 2 * MINIMUM_DEGREE - 1;

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            numbers_of_keys: 0,
            keys: vec![],
            childrens: Vec::new(),
            is_leaf,
        }
    }

    pub fn search(&self, key: &u32) -> Option<&u32> {
        let mut index = 0;
        let mut node_key = self.keys[index];

        while *key > node_key {
            index += 1;

            if index >= self.numbers_of_keys {
                break;
            }

            node_key = self.keys[index];
        }

        if index < self.numbers_of_keys && *key == node_key {
            return self.keys.get(index);
        } else if self.is_leaf {
            return None;
        } else {
            let next_node = &self.childrens[index];
            return next_node.search(key);
        }
    }

    pub fn split_child(&mut self, index: usize) {
        if let Some(child) = self.childrens.get_mut(index) {
            let mut new_node = Self::new(child.is_leaf);
            let new_node_number_of_keys = MINIMUM_DEGREE - 1;
            new_node.numbers_of_keys = new_node_number_of_keys;

            // Move keys[t..] to new node
            // for j = 1 to t - 1
            //   z.key(k) = y.key(j + t)
            // y.n = t - 1
            for j in 0..new_node_number_of_keys {
                let key = child.keys.remove(MINIMUM_DEGREE);
                new_node.keys.insert(j, key);
                child.numbers_of_keys -= 1;
            }

            // Move childrens[t..] to new node if not leaf node
            // if not y.leaf
            //   for j = 1 to t
            //     z.c(j) = y.c(j+t)
            if !child.is_leaf {
                for j in 0..MINIMUM_DEGREE {
                    let nodes = child.childrens.remove(MINIMUM_DEGREE);
                    new_node.childrens.insert(j, nodes);
                }
            }

            // x.key(i) = y.key(t)
            if let Some(key) = child.keys.pop() {
                self.keys.insert(index, key);
                child.numbers_of_keys -= 1;
            }

            // x.c(i+1) = z
            self.childrens.insert(index + 1, Box::new(new_node));

            // x.n = x.n + 1
            self.numbers_of_keys += 1;
        };
    }

    pub fn insert_non_full(&mut self, key: u32) {
        if self.is_leaf {
            match self.keys.binary_search(&key) {
                Ok(_) => (),
                Err(pos) => {
                    self.keys.insert(pos, key);
                }
            }

            self.numbers_of_keys += 1;
        } else {
            let mut index = self.numbers_of_keys - 1;

            while index > 0 && key < self.keys[index] {
                index -= 1;
            }

            if key > self.keys[index] {
                index += 1;
            }

            if self.childrens[index].numbers_of_keys == MAX_DEGREE {
                self.split_child(index);

                if key > self.keys[index] {
                    index += 1
                }
            }

            self.childrens[index].insert_non_full(key)
        }
    }

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        if let Ok(index) = self.keys.binary_search(key) {
            let key = self.keys.remove(index);
            Some(key)
        } else {
            None
        }
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
            let mut node = Node::new(true);
            node.insert_non_full(key);
            self.root = Some(Box::new(node));
        }
    }

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        self.root.as_mut().map_or(None, |node| node.remove(key))
    }

    pub fn get(&self, key: &u32) -> Option<&u32> {
        if let Some(node) = &self.root {
            node.search(key)
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
    #[ignore]
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
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);

        tree.print();

        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&7), Some(&7));
        assert_eq!(tree.get(&8), Some(&8));
        assert_eq!(tree.get(&9), Some(&9));
        assert_eq!(tree.get(&5), Some(&5));
        assert_eq!(tree.get(&10), Some(&10));
        assert_eq!(tree.get(&4), Some(&4));
        assert_eq!(tree.get(&12), None);
    }

    #[test]
    fn delete_on_root_node() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);

        tree.print();
        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.remove(&8), Some(8));
        assert_eq!(tree.remove(&1), None);
        assert_eq!(tree.remove(&8), None);
    }
}
