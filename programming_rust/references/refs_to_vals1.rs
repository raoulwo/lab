use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

// NOTE: References are non-owning pointers that are
// guaranteed to never outlive the value they're referencing.
// There are two types of references:
//
// 1. Shared (immutable) references -> Read-only
// 2. Exclusive (mutable) references -> Read and write
//
// Think of it as *multiple readers*, *single writer*!

// NOTE: Shared references (&T) are Copy -> Assigning the ref
// to another variable creates a copy of that shared ref.
// Mutable refs (&mut T) **aren't** Copy!

fn show(table: &Table) {
    // Iterating over a shared ref (&T) is guaranteed
    // to produce other shared refs as elements/key-value pairs
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table); // `table` is borrowed by the function
    show(&table); // `table` can be used again since it was borrowed, not moved

    {
        // In Rust, references are created explicitly using the `&` operator
        let x = 10;
        let r = &x;
        // References are dereferenced explicitly as well using the `*` operator
        assert_eq!(*r, 10);

        // For mutable refs, use `&mut` instead
        let mut y = 32;
        let m = &mut y;
        // Dereferencing is the same
        *m += 32;
        assert_eq!(*m, 64);
    }

    {
        // The `.` operator implicitly dereferences its left operand
        struct Anime {
            name: &'static str,
            bechdel_pass: bool,
        }

        let anime = Anime {
            name: "Aria: The Animation",
            bechdel_pass: true,
        };
        let anime_ref = &anime;

        // Explicit dereference to access the `name` field
        assert_eq!((*anime_ref).name, "Aria: The Animation");
        // Implicit dereference by default
        assert_eq!(anime_ref.name, "Aria: The Animation");
    }

    {
        // The `.` operator not only implicitly dereferences left operands,
        // it can also implicitly borrow references needed for a method call.

        let mut v = vec![1973, 1968];

        // Explicitly convert `v` into a mutable reference
        (&mut v).sort();
        // Implicitly borrow a `&mut` to `v` since `sort` takes a mutable reference as argument
        v.sort();
    }
}
