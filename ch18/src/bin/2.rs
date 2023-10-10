fn main() {
    let mut hoge = 10;
    let reference = &hoge;
    assert_eq!(*reference, 10);
    *reference = 20;
}