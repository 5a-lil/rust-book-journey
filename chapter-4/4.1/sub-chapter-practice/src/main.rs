fn main() {
    let s1: String = String::from("Salam");
    let s2: String = s1;

    let s2: String = take_then_give(s2);

    println!("{}", s2);
}

fn take_then_give(to_take: String) -> String {
    let to_give: String = to_take;
    to_give
}
