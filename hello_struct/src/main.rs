fn main() {
    let mut user = User {
        active: false,
        username: String::from("John Doe"),
        email: String::from("johndoe@example.com"),
        sign_in_count: 1,
    };

    user.sign_in_count = 2;

    println!("User sign in count is {}", user.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
