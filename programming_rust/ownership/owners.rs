fn main() {
    // Rust enforces the rule that variables have only a single owner.
    // (There are exceptions to this rule)
    print_padovan();

    // Variables own their values, once variables are dropped so are their values.
    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    } // both dropped here

    // The same applies to structs and their fields, as well as vectors, tuples
    // ... with their elements.
    {
        struct Person {
            name: String,
            birth: i32,
        }

        let mut composers = Vec::new();
        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });
        composers.push(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        });

        for composer in &composers {
            println!("{}, born {}", composer.name, composer.birth);
        }
    }
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped here
