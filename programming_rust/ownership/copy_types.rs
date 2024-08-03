fn main() {
    // Primitive types such as integers, chars... aren't moved
    // when re-assigned to other variables, they are copied by default

    // The types which are copied by default are called *Copy Types*

    // The following types are Copy Types:
    //
    // * Integers
    // * Floats
    // * Booleans
    // * Characters
    // * Tuples consisting only of other copy types
    // * Fixed-size arrays consisting only of other copy types

    // NOTE: Rule of thumb -> Only types that don't have to do special
    // clean-up when they are dropped can be *Copy*

    // Structs, Enums **aren't** Copy by default!
    {
        struct Foo {
            number: u32,
        }

        let a = Foo { number: 0 };
        let b = a; // Move value of `a` to `b`

        // println!("{}", a.number);
        // ^ Would cause error
    }

    {
        // Define `Bar` as being Copy
        // This only works for structs/enums holding other Copy types
        #[derive(Copy, Clone)]
        struct Bar {
            number: u32,
        }

        let a = Bar { number: 0 };
        let b = a; // `b` owns copy of `a`

        println!("{}", a.number);
    }
}
