fn main() {
    let reference = &100;
    println!("{:p}", reference);
    assert_eq!(*reference, 100);
}