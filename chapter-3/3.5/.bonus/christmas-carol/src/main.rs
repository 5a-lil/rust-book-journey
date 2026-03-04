const LYRICS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

fn sing(verse: usize) {
    if verse == 12 {
        return;
    }

    println!("On the third day of Christmas, my true love sent to me");

    let mut iterator: usize = verse;
    loop {
        println!("{}", LYRICS[iterator]);
        if iterator == 0 {
            break;
        }
        iterator -= 1;
    }

    println!("\n\n");
    sing(verse + 1);
}

fn main() {
    sing(0);
}
