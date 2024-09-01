// When the owner of a value goes *away*, the value
// is *dropped* by Rust. This can happen when the
// owner goes out of scope, at the end of an expression
// statement... Rust then performs any necessary cleanup
// like memory deallocations. You can customize the
// way Rust operates when *dropping* values by implementing
// the `std::ops::Drop` trait (like a destructor in C++).

struct User {
    name: String,
}

use std::ops::Drop;

impl Drop for User {
    // When a value is dropped, Rust calls this method
    // just before it happens. You can't call this method
    // manually yourself.
    fn drop(&mut self) {
        println!("dropping user {}", self.name);
    }
}

fn main() {
    let _user = User {
        name: "raoulwo".to_string(),
    };
}
