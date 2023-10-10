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