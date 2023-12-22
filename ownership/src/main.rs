fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    println!("{s}"); // error, as ownership is taken

    let x = 5;
    copy(x);
    let y = x; // x is still valid, as i32 is Copy
    println!("{x} {y}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn copy(x: i32) {
    println!("{x}");
}