可変変数も借用できる
```Rust
fn main() {
    let mut hoge = 10;
    let reference = &hoge;
    println!("{}", reference);
}
```

元の変数が可変でも、参照を介して書き換えれない。
```Rust
fn main() {
    let mut hoge = 10;
    let reference = &hoge;
    assert_eq!(*reference, 10);
    *reference = 20;
}
```

借用された変数は、参照のライフタイムが終了するまで書き換えれない
```Rust
fn main() {
    let mut hoge = 10;
    let reference = &hoge;
    println!("{}", reference);
    hoge = 20;
    println!("{}", reference);
}
```

```Rust
fn main() {
    let mut hoge = 10;
    let reference  &hoge;
    println!("{}", reference);
    hoge = 20;
}
```

可変変数も複数回借用できる
```Rust
fn main() {
    let mut hoge = 10;
    let reference1 = &hoge;
    let reference2 = &hoge;
    assert_eq!(*reference1, 10);
    assert_eq!(*reference2, 10);
    hoge = 20;
}
```

## 参照型の可変変数
