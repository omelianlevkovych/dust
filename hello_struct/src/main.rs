fn main() {
    let mut user = User {
        active: false,
        username: String::from("John Doe"),
        email: String::from("johndoe@example.com"),
        sign_in_count: 1,
    };

    user.sign_in_count = 2;

    println!("User sign in count is {}", user.sign_in_count);

    let buildUser = build_user(
        String::from("email"),
        String::from("username"));

    println!("New user email is {}", buildUser.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
