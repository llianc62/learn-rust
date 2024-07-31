/*
// first version
fn fisrt_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn main() {
    let mut s = String::from("hello world");

    let index = fisrt_word(&s);

    s.clear();

    println!("The first word: {}", index);
}
*/

// 兼容 String 类型 和 字面值 (str)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn main() {
    // String
    let sentence = String::from("hello world");

    let word = first_word(&sentence[..]);

    println!("the first word is: {}", word);

    // 字面值（str)
    let sentence = "hello world";

    let word = first_word(&sentence);

    println!("the first word is: {}", word);
}