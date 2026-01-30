use std::ptr;

struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Deque {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }
    fn pop_left(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let node = Box::from_raw(self.head);
                self.head = node.next;
                if !self.head.is_null() {
                    (*self.head).prev = ptr::null_mut();
                } else {
                    self.tail = ptr::null_mut();
                }
                self.len -= 1;
                Some(node.elem)
            }
        }
    }
    fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.tail.is_null() {
                None
            } else {
                let node = Box::from_raw(self.tail);
                self.tail = node.prev;
                if !self.tail.is_null() {
                    (*self.tail).next = ptr::null_mut();
                } else {
                    self.head = ptr::null_mut();
                }
                self.len -= 1;
                Some(node.elem)
            }
        }
    }
    fn append_left(&mut self, elem: T) {
        unsafe {
            let new_head = Box::into_raw(Box::new(Node {
                elem,
                prev: ptr::null_mut(),
                next: self.head,
            }));
            if !self.head.is_null() {
                (*self.head).prev = new_head;
            } else {
                self.tail = new_head;
            }
            self.len += 1;
            self.head = new_head;
        }
    }
    fn append(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
                prev: self.tail,
            }));
            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }
            self.len += 1;
            self.tail = new_tail;
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::Deque;

    #[test]
    fn basics() {
        let mut dq = Deque::new();
        assert_eq!(dq.len(), 0);
        assert!(dq.is_empty());
        assert_eq!(dq.pop(), None);

        dq.append(0);
        assert_eq!(dq.len(), 1);
        assert!(!dq.is_empty());
        assert_eq!(dq.pop_left(), Some(0));
        assert_eq!(dq.len(), 0);
        assert!(dq.is_empty());
        dq.append_left(0);
        assert_eq!(dq.len(), 1);
        assert!(!dq.is_empty());
        assert_eq!(dq.pop(), Some(0));
        assert_eq!(dq.len(), 0);
        assert!(dq.is_empty());
        assert_eq!(dq.pop(), None);

        dq.append(0);
        dq.append(1);
        dq.append(2);
        assert_eq!(dq.pop_left(), Some(0));
        assert_eq!(dq.pop_left(), Some(1));
        assert_eq!(dq.pop_left(), Some(2));
        assert_eq!(dq.len(), 0);
        assert!(dq.is_empty());
        assert_eq!(dq.pop(), None);

        dq.append_left(0);
        dq.append_left(1);
        dq.append_left(2);
        assert_eq!(dq.pop(), Some(0));
        assert_eq!(dq.pop(), Some(1));
        assert_eq!(dq.pop(), Some(2));
        assert_eq!(dq.len(), 0);
        assert!(dq.is_empty());
        assert_eq!(dq.pop(), None);
    }

    #[test]
    fn test_drop() {
        let mut dq = Deque::new();
        dq.append(1);
        dq.append(2);
        dq.append(3);
    }
}
