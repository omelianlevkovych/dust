fn main() {
    println!("Hello, world!");
    print_something(10, 'm');
}

fn print_something(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}
