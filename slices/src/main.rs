fn main() {
    let words = String::from("Hello World !");

    let first_word = first_word(&words);
    println!("First word is {}", first_word);
}

fn first_word(s :&String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    return &s[..]
}