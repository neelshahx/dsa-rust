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

    fn get(&self, i: usize) -> Option<&T> {
        let mut curr = self.head.as_ref();
        for _ in 0..i {
            curr = curr?.next.as_ref();
        }
        curr.map(|node| &node.elem)
    }

    fn insert_head(&mut self, elem: T) {
        let new_head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
        self.head = new_head;
        self.len += 1;
    }

    fn insert_tail(&mut self, elem: T) {
        let mut curr = &mut self.head;
        while let Some(node) = curr {
            curr = &mut node.next;
        }
        *curr = Some(Box::new(Node { elem, next: None }));
        self.len += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i == 0 {
            self.head.take().map(|node| {
                self.head = node.next;
                self.len -= 1;
                node.elem
            })
        } else {
            let mut curr = self.head.as_mut()?;
            for _ in 0..i - 1 {
                curr = curr.next.as_mut()?;
            }
            curr.next.take().map(|node| {
                curr.next = node.next;
                self.len -= 1;
                node.elem
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone> LinkedList<T> {
    fn get_values(&self) -> Vec<T> {
        let mut v = Vec::<T>::new();
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            v.push(node.elem.clone());
            curr = node.next.as_ref();
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut lst: LinkedList<i32> = LinkedList::new();
        assert!(lst.head.is_none());
        assert_eq!(lst.len(), 0);

        lst.insert_head(0);
        assert!(lst.head.is_some());
        assert_eq!(lst.len(), 1);
        if let Some(ref node) = lst.head {
            assert_eq!(node.elem, 0);
        }

        assert_eq!(lst.get(0), Some(&0));
    }

    #[test]
    fn get_values() {
        let mut lst: LinkedList<i32> = LinkedList::new();
        lst.insert_head(5);
        lst.insert_head(4);
        lst.insert_head(3);
        lst.insert_head(2);
        lst.insert_head(1);
        lst.insert_head(0);

        let v: Vec<i32> = (0..=5).collect();
        assert_eq!(lst.get_values(), v);
        assert_eq!(lst.len(), 6);
    }

    #[test]
    fn mix_inserts() {
        let mut lst: LinkedList<i32> = LinkedList::new();
        lst.insert_head(0);
        lst.insert_tail(1);
        lst.insert_head(2);
        lst.insert_tail(3);
        lst.insert_head(4);

        let v = vec![4, 2, 0, 1, 3];
        assert_eq!(lst.get_values(), v);
        assert_eq!(lst.len(), 5);
    }

    #[test]
    fn add_and_remove() {
        let mut lst: LinkedList<i32> = LinkedList::new();
        lst.insert_head(0);
        lst.remove(0);
        assert_eq!(lst.len(), 0);
        assert_eq!(lst.get(0), None);

        let mut lst: LinkedList<i32> = LinkedList::new();
        lst.insert_head(0);
        lst.insert_tail(1);
        lst.remove(0);
        assert_eq!(lst.len(), 1);
        assert_eq!(lst.get(0), Some(&1));
        assert_eq!(lst.get(1), None);

        let mut lst: LinkedList<i32> = LinkedList::new();
        lst.insert_head(0);
        lst.insert_tail(1);
        lst.remove(1);
        assert_eq!(lst.len(), 1);
        assert_eq!(lst.get(0), Some(&0));
        assert_eq!(lst.get(1), None);

        let mut lst: LinkedList<i32> = LinkedList::new();
        lst.insert_head(0);
        lst.insert_tail(1);
        lst.remove(0);
        lst.remove(0);
        assert_eq!(lst.len(), 0);
        lst.insert_head(0);
        lst.insert_tail(1);
        assert_eq!(lst.get_values(), vec![0, 1]);
        assert_eq!(lst.len(), 2);
    }
}

// insert head O(1)
// insert tail O(n)
// get O(n)
// remove O(n)
// getValues() O(n) time and spacek
