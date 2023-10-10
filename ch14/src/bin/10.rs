fn main() {
    let reference;
    {
        let hoge = 100;
        reference = &hoge;
    }
    println!("{}", *reference);
}