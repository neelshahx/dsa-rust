use std::cell::UnsafeCell;

fn main() {
    unsafe {
        let mut data = UnsafeCell::new(10);
        let mref1 = &mut data;
        let sref2 = &*mref1;
        let ptr3 = sref2.get();

        *ptr3 += 3;
        opaque_read(&*sref2.get());
        *sref2.get() += 2;
        *mref1.get() += 1;
        println!("{}", *data.get());
    }
}

fn opaque_read(val: &i32) {
    println!("{}", val);
}

#[cfg(test)]
mod tests {
    //    #[test]
    //    fn stack_borrow_violation() {
    //        unsafe {
    //            let mut data = 1;
    //            let ref1 = &mut data;
    //            let ptr2 = ref1 as *mut _;
    //
    //            *ref1 += 1;
    //            *ptr2 += 2;
    //
    //            println!("{}", data);
    //        }
    //    }

    #[test]
    fn more_complicated_example() {
        unsafe {
            // &mut -> *mut -> &mut -> *mut
            let mut data = 10;
            let ref1 = &mut data; // mutably borrow data
            let ptr2 = ref1 as *mut _; // shared mutable reference (deref = mutable data)
            let ref3 = &mut *ptr2; // same type as ref1
            let ptr4 = ref3 as *mut _; // same type as ptr2

            // *ptr2 += 2; // uncomment to break

            // borrow stack order
            *ptr4 += 4;
            *ref3 += 3;
            *ptr2 += 2;
            *ref1 += 1;
            println!("{}", data);
        }
    }

    #[test]
    fn shared_reference() {
        unsafe {
            let mut data = 10;
            let mref1 = &mut data;
            let ptr2 = mref1 as *mut i32;
            let sref3 = &*mref1;
            let ptr4 = sref3 as *const i32 as *mut i32;

            // stacked borrow order
            opaque_read(&*ptr4);
            opaque_read(sref3);
            *ptr2 += 2;
            *mref1 += 1;

            opaque_read(&data);
        }
    }

    fn opaque_read(val: &i32) {
        println!("{}", val);
    }
}
