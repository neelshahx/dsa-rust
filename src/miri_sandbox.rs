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
    #[test]
    fn stacked_borrows() {
        unsafe {
            let mut data = 1;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;

            *ptr2 += 2;
            *ref1 += 1;

            println!("{}", data);
        }
    }

    #[test]
    fn more_complicated_example() {
        unsafe {
            // &mut -> *mut -> &mut -> *mut
            let mut data = 10;
            let ref1 = &mut data; // mutably borrow data
            let ptr2 = ref1 as *mut _; // shared mutable reference (deref = mutable data)
            let ref3 = &mut *ptr2; // same type as ref1
            let ptr4 = ref3 as *mut _; // same type as ptr2

            // *ptr2 += 2;

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

    fn array_example() {
        unsafe {
            let mut data = [0; 10];
            let slice1 = &mut data[..];
            let (slice2_at_0, slice3_at_1) = slice1.split_at_mut(1);

            let ref4_at_0 = &mut slice2_at_0[0]; // &mut
            let ref5_at_1 = &mut slice3_at_1[0];
            let ptr6_at_0 = ref4_at_0 as *mut i32; // *mut
            let ptr7_at_1 = ref5_at_1 as *mut i32;

            *ptr7_at_1 += 4;
            *ptr6_at_0 += 3;
            *ref5_at_1 += 2;
            *ref4_at_0 += 1;
            println!("{:?}", &data[..]);
        }
    }

    #[test]
    fn array_example2() {
        unsafe {
            let mut data = [0; 10];

            let slice1_all = &mut data[..];
            let ptr2_all = slice1_all.as_mut_ptr();

            let ptr3_at_0 = ptr2_all;
            let ptr4_at_1 = ptr2_all.add(1);
            let ref5_at_0 = &mut *ptr3_at_0;
            let ref6_at_1 = &mut *ptr4_at_1;

            *ref6_at_1 += 6;
            *ref5_at_0 += 5;
            *ptr4_at_1 += 4;
            *ptr3_at_0 += 3;

            for idx in 0..10 {
                *ptr2_all.add(idx) += idx;
            }

            for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
                *elem_ref += idx;
            }

            println!("{:?}", &data[..]);
        }
    }
}
