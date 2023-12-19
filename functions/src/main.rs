fn main() {
    println!("Hello, world!");
    print_something(10);
}

fn print_something(x: i32) {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

