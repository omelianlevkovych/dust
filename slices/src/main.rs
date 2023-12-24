fn main() {
    let words = String::from("Hello World !");

    let index_of_space: usize = first_word(&words);
    println!("first_word: {}", index_of_space);

    for i in 0..index_of_space {
        print!("{}", words.as_bytes()[i] as char);
    }
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}