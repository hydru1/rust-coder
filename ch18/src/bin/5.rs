fn main() {
    let mut hoge = 10;
    let reference1 = &hoge;
    let reference2 = &hoge;
    assert_eq!(*reference1, 10);
    assert_eq!(*reference2, 10);
    hoge = 20;
}