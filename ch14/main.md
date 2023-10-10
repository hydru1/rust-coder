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

### パターンマッチ

パターンマッチもできる(`copied`の型は`reference`と同じ)
```Rust
fn main() {
    let hoge = 10;
    let reference = &hoge;
    let &copied = reference;
    assert_eq!(copied, 10);
    println!("{:p}", &reference);
    println!("{:p}", &copied);
}
```

### シャドーイング
元の変数をシャドーイングしても、参照を通したアクセスは変化しない。
```Rust
fn main() {
    let hoge = 10;
    let reference = &hoge;
    let hoge = 20;
    assert_eq!(hoge, 20);
    assert_eq!(*reference, 10);
}
```

## 未初期化の変数の借用
```Rust
fn main() {
    let hoge;;
    let reference = &hoge; // Error!
    println!("{}", reference);
    hoge = 100;
}
```

条件によって初期化されないパターンもコンパイラが検知してくれる
```Rust
use proconio::input;

fn main() {
    let hoge;
    input! {
        n: i32,
    }
    if n > 0 {
        hoge = 0;
    }
    let reference = &hoge;
}
```

## ライフタイム

```Rust
fn main() {
    let hoge = 100;
    let reference = &hoge; // 借用: ライフタイムの開始
    println!("{}", *reference); // 最後の使用: ライフタイムの終了
}
```

参照元の変数のライフタイムが切れているとエラーになる
```Rust
fn main() {
    let reference;
    {
        let hoge = 100;
        reference = &hoge;
    } // hogeのスコープ終了
    println!("{}", *reference); // ライフタイムが終了した&hogeの使用
}
```

## 一時変数の借用
```Rust
fn main() {
    let hoge = 50;
    let reference = &(hoge + 30);
    println!("{}", reference);
}
```

## 静的なライフタイム
リテラルも借用可能
```Rust
fn main() {
    let reference = &100;
    println!("{:p}", reference);
    assert_eq!(*reference, 100);
}
```

`&100`はstaticなライフタイムを持つ
```Rust
fn main() {
    let reference;
    {
        reference = &100;
    }
    assert_eq!(*reference, 100);
}
```