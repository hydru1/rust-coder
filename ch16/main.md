## 複雑なパターンマッチ
```Rust
fn main() {
    let elements: [(i32, f64); 5] = [
        (6, 12.0),
        (7, 14.0),
        (8, 16.0),
        (9, 18.0),
        (10, 20.0),
    ];

    for &(number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }
}
```

## `ref`パターン
```Rust
fn main() {
    let carbon = (6, 12.0);
    let (number, weight) = &carbon;
    assert_eq!(*number, 6);
    assert_eq!(*weight, 12.0)
}
```