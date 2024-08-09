pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

// Associated functions/methods for structs are defined inside `impl` blocks
impl Queue {
    // NOTE: These methods take `self` as `&mut`, when calling them you don't
    // have to explicitly borrow the mutable ref, it is done so implicitly.

    // NOTE: If you want to define methods which don't mutate the instance in
    // any way, you can pass it as a shared ref. You can also pass it by value,
    // this would move it into the method however.

    // NOTE: You can also define the `self` inside methods to be a smart pointer
    // like `Box<Self>`, `Rc<Self>` or `Arc<Self>`. These methods can only be
    // called on a value of the exact pointer type, when doing so it is moved into
    // the method. A better way would be just to use `&self` and `&mut self` since
    // refs can also be borrowed from smart pointers.

    // `self` refers to the type specified in the `impl` block.
    // In this case `&mut self` is a shorthand for `self: &mut Queue`.
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    // The `this` keyword in languages like C/C++ is implicit.
    // In Rust however, you *must* use `self` to access the associated fields.
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem;
            mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    // Doesn't have a `self` param -> Type-associated function (like statics in OOP languages).
    // `new` is the conventional name for constructor functions.
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    // Associated-const, which is like static variables in other langs (Accessible via Queue::ZERO)
    const ZERO: u32 = 0;
}

fn main() {}
