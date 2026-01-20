fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.extend_from_slice(&[1, 2, 3]);
    let v2: &Vec<i32> = &v1;
    println!("{:?}", v2);
}
