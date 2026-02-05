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

    fn insert(&mut self, key: i32, val: i32) {
        let mut curr = &mut self.root;
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
        let mut curr = &self.root;
        while let Some(n) = curr {
            match &n.left {
                None => return Some(n.val),
                Some(_) => curr = &n.left,
            }
        }
        None
    }

    fn get_max(&self) -> Option<i32> {
        let mut curr = &self.root;
        while let Some(n) = curr {
            match &n.right {
                None => return Some(n.val),
                Some(_) => curr = &n.right,
            }
        }
        None
    }

    fn remove(&mut self, key: i32) -> Option<i32> {
        if let Some(val) = Self::remove_recursive(&mut self.root, key) {
            self.len -= 1;
            return Some(val);
        }
        None
    }

    fn remove_recursive(link: &mut Link, key: i32) -> Option<i32> {
        match link {
            Some(n) if key < n.key => Self::remove_recursive(&mut n.left, key),
            Some(n) if key > n.key => Self::remove_recursive(&mut n.right, key),
            Some(n) => {
                let val = n.val;
                Self::remove_helper(link);
                Some(val)
            }
            None => None,
        }
    }

    fn remove_helper(curr: &mut Link) {
        if let Some(mut n) = curr.take() {
            if n.right.is_some() {
                let mut repl = Self::take_leftmost(&mut n.right);
                repl.left = n.left;
                repl.right = n.right;
                *curr = Some(repl);
            } else {
                *curr = n.left;
            }
        }
    }

    fn take_leftmost(link: &mut Link) -> Box<Node> {
        match link {
            Some(n) if n.left.is_some() => Self::take_leftmost(&mut n.left),
            Some(_) => {
                let mut leftmost = link.take().unwrap();
                *link = leftmost.right.take();
                leftmost
            }
            None => unreachable!(),
        }
    }

    fn inorder_keys_helper(link: &Link, res: &mut Vec<i32>) {
        match link {
            None => (),
            Some(n) => {
                Self::inorder_keys_helper(&n.left, res);
                res.push(n.key);
                Self::inorder_keys_helper(&n.right, res);
            }
        }
    }

    fn inorder_keys(&self) -> Vec<i32> {
        let mut res = Vec::<i32>::new();
        Self::inorder_keys_helper(&self.root, &mut res);
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
            t.insert(k, k + 1);
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
        assert_eq!(tree.inorder_keys(), vec![1, 3, 5]);
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
        assert_eq!(tree.inorder_keys(), vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(tree.len, 7);
    }

    // --- inorder ---

    #[test]
    fn inorder_empty() {
        let tree = empty_tree();
        assert_eq!(tree.inorder_keys(), vec![]);
    }

    #[test]
    fn inorder_single() {
        let mut tree = TreeMap::new();
        tree.insert(5, 0);
        assert_eq!(tree.inorder_keys(), vec![5]);
    }

    #[test]
    fn inorder_tree3() {
        let tree = tree3();
        assert_eq!(tree.inorder_keys(), vec![1, 3, 5]);
    }

    #[test]
    fn inorder_tree7() {
        let tree = tree7();
        assert_eq!(tree.inorder_keys(), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn inorder_skewed_right() {
        let mut tree = right_ladder();
        assert_eq!(tree.inorder_keys(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn inorder_skewed_left() {
        let mut tree = left_ladder();
        assert_eq!(tree.inorder_keys(), vec![1, 2, 3, 4, 5]);
    }

    // -- get --

    #[test]
    fn get() {
        let tree = empty_tree();
        assert!(tree.get(0).is_none());

        let tree = tree3();
        for i in 0..=6 {
            if i % 2 == 0 {
                assert!(tree.get(i).is_none());
            } else {
                assert_eq!(tree.get(i), Some(i + 1));
            }
        }

        let tree = tree7();
        for i in 1..=7 {
            assert!(tree.get(i + 7).is_none());
            assert_eq!(tree.get(i), Some(i + 1));
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

    fn get_min() {
        let tree = empty_tree();
        assert!(tree.get_min().is_none());

        let tree = tree3();
        assert_eq!(tree.get_min(), Some(1));

        let tree = tree7();
        assert_eq!(tree.get_min(), Some(1));

        let tree = left_ladder();
        assert_eq!(tree.get_min(), Some(1));

        let tree = right_ladder();
        assert_eq!(tree.get_min(), Some(1));
    }

    fn get_max() {
        let tree = empty_tree();
        assert!(tree.get_min().is_none());

        let tree = tree3();
        assert_eq!(tree.get_min(), Some(5));

        let tree = tree7();
        assert_eq!(tree.get_min(), Some(7));

        let tree = left_ladder();
        assert_eq!(tree.get_min(), Some(5));

        let tree = right_ladder();
        assert_eq!(tree.get_min(), Some(5));
    }

    // -- remove --

    fn remove() {
        let mut tree = tree7();
        assert!(tree.remove(4).is_some());
        assert_eq!(tree.inorder_keys(), vec![1, 2, 3, 5, 6]);
        assert_eq!(tree.root.as_ref().unwrap().key, 5);
    }
}
