fn main() {
    let mut s1 = String::from("hello");

    let mut len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    change(&mut s1);
    len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}