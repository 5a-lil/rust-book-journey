fn main() {
    let x = 6;
    let y = match x {
        7 => 2,
        6 => 1,
        _ => 0
    };
    println!("{y}");
}
