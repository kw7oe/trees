type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

pub struct BSTree {
    root: Link,
}

impl std::fmt::Debug for BSTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = self.root.as_ref();

        while let Some(n) = node {
            write!(f, "{}", n.val)?;
            node = n.right.as_ref();
        }

        write!(f, "END")
    }
}

impl BSTree {
    pub fn new() -> BSTree {
        BSTree { root: None }
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
                    break;
                } else {
                    temp = n.right.as_mut();
                }
            } else {
                if n.left.is_none() {
                    n.left = Some(Box::new(node));
                    break;
                } else {
                    temp = n.left.as_mut();
                }
            }
        }

        val
    }

    pub fn remove(&mut self, val: i32) -> bool {
        false
    }

    pub fn get(&self, val: &i32) -> Option<&i32> {
        if let Some(n) = self.root.as_ref() {
            if n.val == *val {
                return Some(&n.val);
            }
        }

        None
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

        println!("{:?}", tree);
        // assert_eq!(tree.get(&2), Some(&2));
        // assert_eq!(tree.remove(2), true);
    }
}
