fn main() {
    let mut hoge = 10;
    let reference = &hoge;
    println!("{}", reference);
    hoge = 20;
    println!("{}", reference);
}