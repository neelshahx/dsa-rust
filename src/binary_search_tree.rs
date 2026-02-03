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

    fn dfs(node: &Link, key: i32) -> &Link {
        match node {
            None => node,
            Some(n) => {
                if key > n.key {
                    if n.right.is_some() {
                        Self::dfs(&n.right, key)
                    } else {
                        node
                    }
                } else if key < n.key {
                    if n.left.is_some() {
                        Self::dfs(&n.left, key)
                    } else {
                        node
                    }
                } else {
                    node
                }
            }
        }
    }

    // O(log(n))
    fn insert(&mut self, key: i32, val: i32) {
        //        let new_node = Some(Box::new(Node {
        //            key,
        //            val,
        //            left: None,
        //            right: None,
        //        }));
        //        if let Some(root) = self.root {
        //            let parent = Self::dfs(self.root, key);
        //            if key > parent.key {
        //                parent.right = new_node;
        //            } else {
        //                parent.left = new_node;
        //            }
        //        } else {
        //            self.root = new_node;
        //        }
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

    fn get_inorder_keys(&self) -> Vec<i32> {
        vec![1, 2, 3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf_node(key: i32) -> Link {
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
        let left = leaf_node(1);
        let right = leaf_node(5);
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
        let tree = empty_tree();
        assert_eq!(TreeMap::dfs(&tree.root, 5), &tree.root);
    }
}
