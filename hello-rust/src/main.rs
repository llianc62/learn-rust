use ferris_says::say;
use std::io::{stdout, BufWriter};

fn greet_world() {
    let english = "Hello World!";
    let chinese = "你好，世界！";
    let regions = [chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    greet_world();
}
