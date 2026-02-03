#![allow(unused)]
// design notes
// maps integer keys to integer values

struct TreeMap {
    root: Link,
    len: usize,
}

type Link = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
struct Node {
    key: i32,
    val: i32,
    left: Link,
    right: Link,
}

impl TreeMap {
    fn new() -> Self {
        Self { root: None, len: 0 }
    }

    // returns mutable reference to location where need to insert the value
    fn find(node: &mut Link, key: i32) -> &mut Link {
        let mut current = node;
        loop {
            let direction = match current.as_ref() {
                None => return current,
                Some(n) => {
                    if key > n.key {
                        1
                    } else if key < n.key {
                        -1
                    } else {
                        0
                    }
                }
            };
            match direction {
                1 => {
                    current = &mut current.as_mut().unwrap().right;
                }
                -1 => {
                    current = &mut current.as_mut().unwrap().left;
                }
                _ => return current,
            }
        }
    }

    // if key in tree, then replace val
    // if key not in tree, find parent, then insert leaf node
    // if tree empty, set to root
    fn insert(&mut self, key: i32, val: i32) {
        let link = Self::find(&mut self.root, key);
        match link {
            Some(n) => n.val = val,
            None => {
                *link = Some(Box::new(Node {
                    key,
                    val,
                    left: None,
                    right: None,
                }));
                self.len += 1;
            }
        }
    }

    fn get(&self, key: i32) -> Option<i32> {
        None
    }

    fn get_min(&self) -> Option<i32> {
        None
    }

    fn get_max(&self) -> Option<i32> {
        None
    }

    fn remove(&self, key: i32) {}

    fn inorder_keys(link: &Link, res: &mut Vec<i32>) {
        match link {
            None => (),
            Some(n) => {
                Self::inorder_keys(&n.left, res);
                res.push(n.key);
                Self::inorder_keys(&n.right, res);
            }
        }
    }

    fn get_inorder_keys(&self) -> Vec<i32> {
        let mut res = Vec::<i32>::new();

        // recursive implementation
        // Self::inorder_keys(&self.root, &mut res);

        let mut stk = Vec::<&Node>::new();
        // current node is last stop on left subtree, last place to be visited, bottom of stack
        let mut current = &self.root;
        loop {
            // pointers are added to stack, so if right leaf node, it will be popped next
            while let Some(n) = current {
                stk.push(n);
                current = &n.left;
            }
            // on visit, you add key and update pointer to right subtree
            match stk.pop() {
                None => break,
                Some(n) => {
                    res.push(n.key);
                    current = &n.right;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_tree() -> TreeMap {
        TreeMap { root: None, len: 0 }
    }

    fn tree3() -> TreeMap {
        let mut t = TreeMap::new();
        t.insert(3, 0);
        t.insert(1, 0);
        t.insert(5, 0);
        t
    }

    //       4
    //      / \
    //     2   6
    //    / \ / \
    //   1  3 5  7
    fn tree7() -> TreeMap {
        let mut t = TreeMap::new();
        for k in [4, 2, 6, 1, 3, 5, 7] {
            t.insert(k, 0);
        }
        t
    }

    // --- find ---

    #[test]
    fn find_empty() {
        let mut tree = empty_tree();
        assert!(TreeMap::find(&mut tree.root, 5).is_none());
    }

    #[test]
    fn find_exact() {
        let mut tree = tree3();
        assert_eq!(TreeMap::find(&mut tree.root, 3).as_ref().unwrap().key, 3);
        assert_eq!(TreeMap::find(&mut tree.root, 1).as_ref().unwrap().key, 1);
        assert_eq!(TreeMap::find(&mut tree.root, 5).as_ref().unwrap().key, 5);
    }

    #[test]
    fn find_missing_returns_none() {
        let mut tree = tree3(); // 1 3 5
        // keys not in tree should return None (the empty slot)
        assert!(TreeMap::find(&mut tree.root, 0).is_none());
        assert!(TreeMap::find(&mut tree.root, 2).is_none());
        assert!(TreeMap::find(&mut tree.root, 4).is_none());
        assert!(TreeMap::find(&mut tree.root, 6).is_none());
    }

    // --- insert ---

    #[test]
    fn insert_single() {
        let mut tree = TreeMap::new();
        tree.insert(42, 10);
        assert_eq!(tree.root.as_ref().unwrap().key, 42);
        assert_eq!(tree.root.as_ref().unwrap().val, 10);
        assert_eq!(tree.len, 1);
    }

    #[test]
    fn insert_builds_tree() {
        let tree = tree3();
        assert_eq!(tree.get_inorder_keys(), vec![1, 3, 5]);
        assert_eq!(tree.len, 3);
    }

    #[test]
    fn insert_duplicate_replaces_val() {
        let mut tree = tree3();
        tree.insert(3, 99);
        assert_eq!(tree.root.as_ref().unwrap().val, 99);
        assert_eq!(tree.len, 3); // len unchanged
    }

    #[test]
    fn insert_larger_tree() {
        let tree = tree7();
        assert_eq!(tree.get_inorder_keys(), vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(tree.len, 7);
    }

    // --- inorder ---

    #[test]
    fn inorder_empty() {
        let tree = empty_tree();
        assert_eq!(tree.get_inorder_keys(), vec![]);
    }

    #[test]
    fn inorder_single() {
        let mut tree = TreeMap::new();
        tree.insert(5, 0);
        assert_eq!(tree.get_inorder_keys(), vec![5]);
    }

    #[test]
    fn inorder_tree3() {
        let tree = tree3();
        assert_eq!(tree.get_inorder_keys(), vec![1, 3, 5]);
    }

    #[test]
    fn inorder_tree7() {
        let tree = tree7();
        assert_eq!(tree.get_inorder_keys(), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn inorder_skewed_right() {
        let mut tree = TreeMap::new();
        for k in [1, 2, 3, 4, 5] {
            tree.insert(k, 0);
        }
        assert_eq!(tree.get_inorder_keys(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn inorder_skewed_left() {
        let mut tree = TreeMap::new();
        for k in [5, 4, 3, 2, 1] {
            tree.insert(k, 0);
        }
        assert_eq!(tree.get_inorder_keys(), vec![1, 2, 3, 4, 5]);
    }
}
