#![allow(unused)]

use std::alloc::{Layout, alloc, dealloc};

struct ArrayList {
    ptr: *mut i32,
    len: usize,
    capacity: usize,
}

impl ArrayList {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = if capacity == 0 {
            std::ptr::null_mut()
        } else {
            unsafe {
                let layout = Layout::array::<i32>(capacity).unwrap();
                alloc(layout) as *mut i32 // convert from *mut u8 to *mut i32
            }
        };

        Self {
            ptr,
            len: 0,
            capacity,
        }
    }
}

impl Drop for ArrayList {
    fn drop(&mut self) {
        if self.capacity != 0 {
            unsafe {
                let layout = Layout::array::<i32>(self.capacity).unwrap();
                dealloc(self.ptr as *mut u8, layout); // convert from *mut i32 to *mut u8
            }
        }
    }
}
