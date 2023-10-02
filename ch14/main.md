## 参照
変数に`&`をつけると、その変数の先頭アドレスが得られる。

フォーマット文字列で`{:p}`と書くと、アドレスを出力できる(`{}`、`{:?}`はそのままの値)
```Rust
fn main() {
    let hoge: i8 = 100;
    println!("{:p}", &hoge);
}
// 0xb3b5b9f6cf
```

## 型推論

型推論は、参照、参照外しを超えて行われる。
```Rust
fn main() {
    let hoge = 100;
    let reference = &hoge;
    assert_eq!(*reference, 100_i8);
}
```

## 参照外し
`*`を使うと参照を外せる

`+``-``*``/``%`のような演算子は、参照外しをしなくても使うことができる。
```Rust
fn main() {
    let hoge: i8 = 100;
    let reference = &hoge;
    assert_eq!(reference + 1_i8, 101_i8);
}
```

`println!`で`*reference`の値を出力できる
```Rust
fn main() {
    let hoge = 100;
    let reference = &hoge;
    println!("{}", reference);
}
```

タプルに対して、`reference.0`で`(*reference).0`の値を出力できる
```Rust
fn main() {
    let tuple: (i32, f64) = (10, 3.14);
    let reference = &tuple;
    assert_eq!(reference.0, 10_i32);
    assert_eq!(reference.1, 3.14_f64);
}
```

[ポインタを出力したい](./as_raw_bytes.rs)