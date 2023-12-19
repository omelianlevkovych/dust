fn main() {
    let a = [1,2,3,4,5];

    for element in a {
        println!("the value is: {}", element);
    }

    // Reversed
    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }
}
