use std::collections::HashMap;

fn main() {
    let mut num_vec = std::vec![1, 2, 9, 77, 3, 8];
    num_vec.sort();

    let len = num_vec.len();
    let mut map = HashMap::new();

    for num in &num_vec {
        *(map.entry(num).or_insert(0)) += 1;
    }
    println!("{map:?}");
    if let Some(max) = map.values().max() {
        for (key, val) in &map {
            if val == max {
                println!("{}", key);
                break;
            }
        }
    }
    
    println!("{num_vec:?}");
    if num_vec.len() % 2 == 0 {
        println!("{}", (num_vec[len / 2 - 1] + num_vec[len / 2]) / 2);
    } else {
        println!("{}", num_vec[len / 2]);
    }

}
