#![allow(unused)]

struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn insert_head(&mut self, elem: T) {
        let new_head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
        self.head = new_head;
        self.len += 1;
    }

    fn get(&self, i: usize) -> Option<&T> {
        let mut curr = self.head.as_ref();
        for _ in 0..i {
            curr = curr?.next.as_ref();
        }
        curr.map(|node| &node.elem)
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut lst: LinkedList<i32> = LinkedList::new();
        assert_eq!(lst.len, 0);
        assert!(lst.head.is_none());

        lst.insert_head(0);
        if let Some(ref node) = lst.head {
            assert_eq!(node.elem, 0);
        }
    }
}

// insert head O(1)
// insert tail O(n)
// get O(n)
// remove O(n)
// getValues() O(n) time and spacek
