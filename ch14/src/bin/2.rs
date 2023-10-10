fn main() {
    let hoge: i8 = 100;
    let reference = &hoge;
    assert_eq!(reference + 1_i8, 101_i8);
}