fn celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    (celsius_temp * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit_temp: f64) -> f64 {
    (fahrenheit_temp - 32.0) / 9.0 * 5.0
}

fn main() {
    println!("{}", celsius_to_fahrenheit(5.0));
    println!("{}", fahrenheit_to_celsius(5.0));
}


