fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // let v = vec![100, 32, 57];
    for i in v {
        println!("{i}");
    }

    v.push(5);
}
