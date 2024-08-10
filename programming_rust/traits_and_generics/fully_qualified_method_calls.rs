fn main() {
    // Methods are just special kinds of functions,
    // the lines below are equivalent. You just pass
    // `self` as an argument to the first parameter.
    let foo = "foo".to_string();
    let bar = str::to_string("bar");

    // Because `to_string` is a method of the `ToString`
    // trait there are two more ways of calling it:
    let foobar = ToString::to_string("foobar");
    let barfoo = <str as ToString>::to_string("barfoo");

    // NOTE: Forms 2, 3, 4 are *qualified* method calls.
    // Form 4 is a *fully qualified* method call.

    // NOTE: When calling a method the simple way (form 1),
    // Rust follows its own method lookup algorithm to figure
    // out which method should be called. With the other forms,
    // you can specify explicitly which method should be called.
    // You usually do this in one of the following scenarios:
    //
    // 1. There are two methods with the same name.
    // 2. Rust can't infer the type of `self`.
    // 3. The function itself is used as a value.

    // NOTE: Fully qualified syntax also works for generics!
}
