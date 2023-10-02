fn main() {
    let hoge = 100;
    let reference = &hoge;
    assert_eq!(*reference, 100_i8);
}