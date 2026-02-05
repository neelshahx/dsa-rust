#[derive(Debug)]
struct TreeMap {
    root: Link,
    len: usize,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
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

    // double break feels inelegant
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

    // some(some(...)) feels wrong
    fn get_keys_inorder(&self) -> Vec<i32> {
        let mut res = Vec::<i32>::new();
        let mut stk = Vec::<&Link>::new();
        let mut curr = &self.root;
        loop {
            while let Some(n) = curr {
                stk.push(curr);
                curr = &n.left;
            }
            match stk.pop() {
                Some(Some(n)) => {
                    res.push(n.key);
                    curr = &n.right;
                }
                _ => break,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //    3
    //   / \
    //  1   5
    fn three_tree() -> TreeMap {
        let mut tree = TreeMap::new();
        tree.insert(3, 4);
        tree.insert(1, 2);
        tree.insert(5, 6);
        tree
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

    #[test]
    fn empty_three_tree_inorder() {
        let tree = TreeMap::new();
        assert_eq!(tree.get_keys_inorder(), vec![]);

        let tree = three_tree();
        assert_eq!(tree.get_keys_inorder(), vec![1, 3, 5]);
    }

    #[test]
    fn right_ladder_inorder() {
        let tree = right_ladder();
        assert_eq!(tree.get_keys_inorder(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn left_ladder_inorder() {
        let tree = left_ladder();
        assert_eq!(tree.get_keys_inorder(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn get() {
        let tree = TreeMap::new();
        assert!(tree.get(0).is_none());

        let tree = three_tree();
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
