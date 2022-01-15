use std::collections::VecDeque;

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

pub struct BSTree {
    root: Link,
    size: u32,
}

impl BSTree {
    pub fn new() -> BSTree {
        BSTree {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, val: i32) -> i32 {
        let node = Node {
            val,
            left: None,
            right: None,
        };

        if self.root.is_none() {
            self.root = Some(Box::new(node));

            return val;
        }

        // Take node out of root,
        // now root is None
        let mut temp = self.root.as_mut();

        // Find the right node to insert in,
        // either left or right.
        while let Some(n) = temp {
            if val > n.val {
                if n.right.is_none() {
                    n.right = Some(Box::new(node));
                    self.size += 1;
                    break;
                } else {
                    temp = n.right.as_mut();
                }
            } else {
                if n.left.is_none() {
                    n.left = Some(Box::new(node));
                    self.size += 1;
                    break;
                } else {
                    temp = n.left.as_mut();
                }
            }
        }

        val
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if self.root.is_none() {
            return false;
        }

        let mut node = self.root;
        let mut prev = self.root.unwrap();

        while let Some(n) = node {
            // Found the node to remove.
            //
            // Actor invovled: Node Parents and Children
            if n.val == val {
                if let Some(left) = &prev.left {
                    if left.val == val {
                        prev.left = n.left.take()
                    }
                }

                return true;
            }

            if n.val > val {
                node = n.left;
            } else {
                node = n.right;
            }

            prev = n;
        }

        false
    }

    pub fn get(&self, val: &i32) -> Option<&i32> {
        let mut node = self.root.as_ref();

        while let Some(n) = node {
            if &n.val == val {
                return Some(&n.val);
            }

            if &n.val > val {
                node = n.left.as_ref();
            } else {
                node = n.right.as_ref();
            }
        }

        None
    }

    pub fn print(&self) {
        if self.root.is_none() {
            println!("None");
        }

        let mut queue: VecDeque<&Box<Node>> = VecDeque::new();
        let node = self.root.as_ref().unwrap();
        queue.push_back(node);

        while !queue.is_empty() {
            if let Some(n) = queue.pop_front() {
                println!("{}", n.val);

                if let Some(left) = &n.left {
                    queue.push_back(left);
                }

                if let Some(right) = &n.right {
                    queue.push_back(right);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BSTree;

    #[test]
    fn basics() {
        let mut tree = BSTree::new();

        assert_eq!(tree.insert(2), 2);
        assert_eq!(tree.insert(3), 3);
        assert_eq!(tree.insert(5), 5);
        assert_eq!(tree.insert(4), 4);
        assert_eq!(tree.insert(1), 1);

        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&3), Some(&3));
        assert_eq!(tree.get(&6), None);

        assert_eq!(tree.remove(2), true);
        assert_eq!(tree.get(&2), None);
    }

    // tree.insert(8);
    // tree.insert(9);
    // tree.insert(4);
    // tree.insert(10);
    // tree.insert(3);
    // tree.insert(5);
}
