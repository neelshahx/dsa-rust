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
}
