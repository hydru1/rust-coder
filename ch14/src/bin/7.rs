#![allow(warnings)]
#![allow(wa)]

fn main() {
    let hoge;;
    let reference = &hoge;
    println!("{}", reference);
    hoge = 100;
}