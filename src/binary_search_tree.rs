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

    // only return None if None passed in.
    // if key not present: return parent node for insertion,
    // else return existing node with matching key.
    // caller has to do one more comparison before insertion
    fn dfs(node: &mut Link, key: i32) -> &mut Link {
        let mut current = node;
        loop {
            let direction = match current.as_ref() {
                None => return current,
                Some(n) => {
                    if key > n.key && n.right.is_some() {
                        1
                    } else if key < n.key && n.left.is_some() {
                        -1
                    } else {
                        0
                    }
                }
            };
            match direction {
                1 => current = &mut current.as_mut().unwrap().right,
                -1 => current = &mut current.as_mut().unwrap().left,
                _ => return current,
            }
        }
    }

    // O(log(n))
    fn insert(&mut self, key: i32, val: i32) {
        // new node is either a leaf node or we do a value replacement (key match)
        match Self::dfs(&mut self.root, key) {
            None => {
                self.root = Some(Box::new(Node {
                    key,
                    val,
                    left: None,
                    right: None,
                }));
                self.len += 1;
            }
            Some(p) => {
                if p.key == key {
                    p.val = val;
                } else {
                    let new_link = Some(Box::new(Node {
                        key,
                        val,
                        left: None,
                        right: None,
                    }));
                    if key > p.key {
                        p.right = new_link;
                    } else {
                        p.left = new_link;
                    }
                    self.len += 1;
                }
            }
        };
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

    fn leaf_link(key: i32) -> Link {
        Some(Box::new(Node {
            key,
            val: 0,
            left: None,
            right: None,
        }))
    }

    fn empty_tree() -> TreeMap {
        TreeMap { root: None, len: 0 }
    }

    fn tree3() -> TreeMap {
        let left = leaf_link(1);
        let right = leaf_link(5);
        let root = Some(Box::new(Node {
            key: 3,
            val: 0,
            left,
            right,
        }));
        TreeMap { root, len: 3 }
    }

    #[test]
    fn dfs_empty() {
        let mut tree = empty_tree();
        assert!(TreeMap::dfs(&mut tree.root, 5).is_none());
    }

    #[test]
    fn dfs_tree3() {
        let mut tree = tree3(); // 3 1 5
        assert_eq!(TreeMap::dfs(&mut tree.root, 4).as_ref().unwrap().key, 5);
        assert_eq!(TreeMap::dfs(&mut tree.root, 6).as_ref().unwrap().key, 5);
        assert_eq!(TreeMap::dfs(&mut tree.root, 2).as_ref().unwrap().key, 1);
        assert_eq!(TreeMap::dfs(&mut tree.root, 0).as_ref().unwrap().key, 1);
        assert_eq!(TreeMap::dfs(&mut tree.root, 3).as_ref().unwrap().key, 3);
    }

    #[test]
    fn make_tree3() {
        let mut tree = TreeMap { root: None, len: 0 };
        tree.insert(3, 0);
        tree.insert(1, 0);
        tree.insert(5, 0);
        assert_eq!(tree.root.as_ref().unwrap().key, 3);
        let left = tree.root.as_ref().unwrap().left.as_ref();
        assert_eq!(left.unwrap().key, 1);
        let right = tree.root.as_ref().unwrap().right.as_ref();
        assert_eq!(right.unwrap().key, 5);
        assert_eq!(tree.len, 3);
    }

    #[test]
    fn inorder_traversal() {
        let tree = tree3();
        assert_eq!(tree.get_inorder_keys(), vec![1, 3, 5]);
    }
}
