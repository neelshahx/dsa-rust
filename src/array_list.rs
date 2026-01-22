#![allow(unused)]

use std::{
    alloc::{Layout, alloc, dealloc},
    array,
};

struct ArrayList {
    ptr: *mut i32,
    len: usize,
    capacity: usize,
}

// invariant: len <= capacity
impl ArrayList {
    pub fn new() -> Self {
        // len = capacity = 0
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        // len = 0, capacity >= 0
        let ptr = if capacity == 0 {
            std::ptr::null_mut()
        } else {
            unsafe {
                let layout = Layout::array::<i32>(capacity).unwrap();
                alloc(layout) as *mut i32
            }
        };

        Self {
            ptr,
            len: 0,
            capacity,
        }
    }

    pub fn get(&self, i: usize) -> i32 {
        // len, capacity unch'd
        assert!(i < self.len);
        unsafe { *(self.ptr.wrapping_add(i)) }
    }

    pub fn set(&mut self, i: usize, n: i32) {
        // len, capacity unch'd
        assert!(i < self.len);
        self._set(i, n);
    }

    pub fn _set(&mut self, i: usize, n: i32) {
        // len, capacity unch'd
        unsafe {
            *(self.ptr.wrapping_add(i)) = n;
        }
    }

    pub fn popback(&mut self) -> i32 {
        // before: 1 <= len <= capacity
        // after: 0 <= len - 1 <= capacity
        assert!(self.len > 0);
        let i = self.len - 1;
        let v = self.get(i);
        self.len -= 1;
        v
    }

    pub fn pushback(&mut self, n: i32) {
        // before: len < capacity
        // after: len + 1 <= capacity
        if (self.len == self.capacity) {
            self.resize()
        }
        self._set(self.len, n);
        self.len += 1;
    }

    pub fn resize(&mut self) {
        // len <= capacity <= 2*capacity
        let new_capacity = if self.capacity == 0 {
            2
        } else {
            2 * self.capacity
        };
        let ptr = unsafe {
            let layout = Layout::array::<i32>(new_capacity).unwrap();
            alloc(layout) as *mut i32
        };
        for i in 0..self.len {
            unsafe {
                *ptr.wrapping_add(i) = *self.ptr.wrapping_add(i);
            }
        }
        if self.capacity > 0 {
            let old_ptr = self.ptr;
            let old_layout = Layout::array::<i32>(self.capacity).unwrap();
            unsafe {
                dealloc(old_ptr as *mut u8, old_layout);
            }
        }
        self.ptr = ptr;
        self.capacity = new_capacity;
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.len
    }
}

impl Drop for ArrayList {
    fn drop(&mut self) {
        if self.capacity != 0 {
            unsafe {
                // if (!self.ptr.is_null()) {
                let layout = Layout::array::<i32>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout); // convert from *mut i32 to *mut u8
                // }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let a = ArrayList::new();
        assert_eq!(a.size(), 0);
        assert_eq!(a.capacity(), 0);
    }

    #[test]
    fn test_set_get() {
        let mut a = ArrayList::with_capacity(2);
        a.pushback(9);
        a.pushback(9);
        assert_eq!(a.get(0), 9);
        assert_eq!(a.get(1), 9);
        a.set(0, 1);
        a.set(1, 2);
        assert_eq!(a.get(0), 1);
        assert_eq!(a.get(1), 2);
    }

    #[test]
    fn test_resize() {
        let mut a = ArrayList::new();
        assert_eq!(a.capacity(), 0);
        a.pushback(1);
        assert_eq!(a.capacity(), 2);
        a.pushback(2);
        assert_eq!(a.get(0), 1);
        assert_eq!(a.get(1), 2);
        assert_eq!(a.size(), 2);
        assert_eq!(a.capacity(), 2);
        a.resize();
        assert_eq!(a.get(0), 1);
        assert_eq!(a.get(1), 2);
        assert_eq!(a.size(), 2);
        assert_eq!(a.capacity(), 4);
    }

    #[test]
    fn test_pushback_popback() {
        let mut a = ArrayList::new();
        a.pushback(0);
        a.pushback(1);
        a.pushback(2);
        a.pushback(3);
        a.pushback(4);
        a.pushback(5);
        assert_eq!(a.capacity(), 8);
        assert_eq!(a.size(), 6);
        assert_eq!(a.popback(), 5);
        assert_eq!(a.popback(), 4);
        assert_eq!(a.popback(), 3);
        assert_eq!(a.popback(), 2);
        assert_eq!(a.popback(), 1);
        assert_eq!(a.popback(), 0);
        assert_eq!(a.capacity(), 8);
        assert_eq!(a.size(), 0);
    }
}
