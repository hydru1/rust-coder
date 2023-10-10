fn main() {
    let carbon = (6, 12.0);
    let (number, weight) = &carbon;
    assert_eq!(*number, 6);
    assert_eq!(*weight, 12.0)
}