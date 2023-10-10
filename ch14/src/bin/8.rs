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