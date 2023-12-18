fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
}
