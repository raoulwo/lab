struct Account {
    username: String,
    is_new: bool,
}

fn greet_new_user(user: &Account) {
    println!("Hello, {}", user.username);
}

fn main() {
    let user = Account {
        username: "raoulwo".into(),
        is_new: true,
    };

    match user {
        Account {
            // In order to pass a ref to `user` in the function below, we have to match
            // `username` using the `ref` keyword so that its value is passed as a ref,
            // else it would be moved out of `user` because it is a `String` making
            // `user` unusable afterwards.
            ref username: username,
            // NOTE: You can use `ref mut` to borrow mutably.
            is_new: true,
        } => {
            greet_new_user(&user);
        }
        _ => {}
    }
}
