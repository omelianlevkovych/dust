fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}"); // error: value borrowed here after move
}
