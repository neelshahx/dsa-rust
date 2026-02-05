#[derive(Debug)]
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

    fn insert(&mut self, key: i32, val: i32) {
        let mut curr = &mut self.root; // pointer
        loop {
            match curr {
                Some(n) => {
                    if key < n.key {
                        curr = &mut n.left;
                    } else if key > n.key {
                        curr = &mut n.right;
                    } else {
                        n.val = val;
                        break;
                    }
                }
                None => {
                    // why do i need to dereference curr here?
                    *curr = Some(Box::new(Node {
                        key,
                        val,
                        left: None,
                        right: None,
                    }));
                    self.len += 1;
                    break;
                }
            };
        }
    }

    fn get(&self, key: i32) -> Option<i32> {
        let mut res = None;
        let mut curr = &self.root;
        while let Some(n) = curr {
            if key > n.key {
                curr = &n.right;
            } else if key < n.key {
                curr = &n.left;
            } else {
                res = Some(n.val);
                break;
            }
        }
        res
    }

    fn get_min(&self) -> Option<i32> {
        None
    }

    fn get_max(&self) -> Option<i32> {
        None
    }


    // returns value
    fn remove(&mut self, key: i32) -> Option<i32> {
        let mut res = None;
        let mut prev: &mut Link;
        let mut curr = &mut self.root;
        while let Some(n) = curr {
            if key > n.key {
                prev = curr;
                curr = &mut n.right;
            } else if key < n.key {
                prev = curr;
                curr = &mut n.left;
            } else {
                res = Self::_remove(prev, curr, &mut n.left, &mut n.right);
            }
        }
        res
    }

    fn _remove(par: &mut Link, curr: &mut Link, left: &mut Link, right: &mut Link) -> Option<i32> {
        Some(1)
    }

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
        Self::inorder_keys(&self.root, &mut res);
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
        t.insert(3, 4);
        t.insert(1, 2);
        t.insert(5, 6);
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

    fn right_ladder() -> TreeMap {
        let mut tree = TreeMap::new();
        for i in [1, 2, 3, 4, 5] {
            tree.insert(i, i + 1);
        }
        tree
    }

    fn left_ladder() -> TreeMap {
        let mut tree = TreeMap::new();
        for i in [5, 4, 3, 2, 1] {
            tree.insert(i, i + 1);
        }
        tree
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
        let mut tree = right_ladder();
        assert_eq!(tree.get_inorder_keys(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn inorder_skewed_left() {
        let mut tree = left_ladder();
        assert_eq!(tree.get_inorder_keys(), vec![1, 2, 3, 4, 5]);
    }

    // -- get --

    #[test]
    fn get() {
        let tree = TreeMap::new();
        assert!(tree.get(0).is_none());

        let tree = tree3();
        for i in 0..=6 {
            if i % 2 == 0 {
                assert!(tree.get(i).is_none());
            } else {
                assert_eq!(tree.get(i), Some(i + 1));
            }
        }

        let tree = left_ladder();
        for i in 1..=5 {
            assert!(tree.get(i + 5).is_none());
            assert_eq!(tree.get(i), Some(i + 1));
        }

        let tree = right_ladder();
        for i in 1..=5 {
            assert!(tree.get(i + 5).is_none());
            assert_eq!(tree.get(i), Some(i + 1));
        }
    }
}
