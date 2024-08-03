fn main() {
    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]; // allocated here
        let t = s; // moved to `t` -> `t` now *owns* the vector
        let u = t; // moved to `u` -> `u` now *owns* the vector
    } // dropped here

    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]; // allocated here
        let t = s.clone(); // `t` owns a deep copy of the vector
        let u = s.clone(); // `u` owns a deep copy of the vector
    } // dropped here

    {
        let mut s = "foo".to_string();
        s = "bar".to_string(); // value "foo" dropped here
    }

    {
        let mut s = "foo".to_string();
        let t = s; // `t` now owns value of `s`, `s` is considered uninitialized
        s = "bar".to_string(); // nothing dropped here
    }

    // Moves also occur when:
    //
    // 1. Passing arguments to functions
    // 2. Returning function values
    // 3. Constructing new values (such as structs, collections...)

    // NOTE: The moves always work on the stack allocated values,
    // not the heap allocated memory. A move on a Vec<T> would only
    // work on three machine-words (buffer-pointer, capacity, length).

    {
        let v = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];

        for mut s in v {
            // The loop *consumes* the vector, since the elements are moved out
            s.push('!');
            println!("{}", s);
        }

        // All elements of `v` are moved out, leaving `v` uninitialized
    }
}
