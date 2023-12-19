fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("{count}");
        let mut remaining = 10;

        loop {
            println!("{remaining}");
            if remaining == 9 {
                break;
            }
            if count == 10 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");
}
