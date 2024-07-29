use std::io;

fn main() {
    println!("Please input current Celsius degree(℃): ");

    let mut cel_deg = String::new();

    io::stdin()
        .read_line(&mut cel_deg)
        .expect("Failed to read line");

    let cel_deg: f64 = cel_deg.trim().parse().expect("invalid input");

    let fah_deg: f64 = (cel_deg * 9.0 / 5.0) + 32.0;
    println!("current Fahrenheit degree: {fah_deg}℉");
}
