fn main() {
    let hoge = 10;
    let reference = &hoge;
    let hoge = 20;
    assert_eq!(hoge, 20);
    assert_eq!(*reference, 10);
}