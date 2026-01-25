use std::rc::Rc;

fn main() {
    {
        let mut v1: Vec<i32> = Vec::new();
        v1.extend_from_slice(&[1, 2, 3]);
        let v2: &Vec<i32> = &v1;
        assert_eq!(v1, vec![1, 2, 3]); // v1 and v2 point to same thing
        assert_eq!(*v2, vec![1, 2, 3]);
    }

    {
        let mut v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = v1.clone();
        v1[0] = 2;
        assert_eq!(v1, vec![2, 2, 3]); // v1 and v2 diverge 
        assert_eq!(v2, vec![1, 2, 3]);
    }

    {
        let v2 = {
            let v1: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3]);
            v1.clone()
        };
        println!("{:?}", v2); // v2 can outlive v1
    }

    // not allowed
    // {
    //     let v2: &Vec<i32> = {
    //         let v1: Vec<i32> = vec![1, 2, 3];
    //         &v1
    //     };
    //     println!("{:?}", v2); // v2 can outlive v1
    // }
}
