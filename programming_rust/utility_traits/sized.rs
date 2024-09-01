// *Sized types* are types whose values all have
// the same size in memory. Almost all types in
// Rust are sized, those that are implement the
// `std::marker::Sized` trait with no methods
// or associated functions. Rust implements this
// automatically for all sized types, you can't
// do this yourself. Like its module suggests it
// simply acts as a *marker trait* and its only
// use is for certain trait bounds which should
// only apply for generic types with a known size
// at compile time.
//
// Rust also has *unsized types* such as string
// strices `str` or slices of T `[T]`. Because
// they're unsized we use references when working
// with those types. Dynamic trait objects denoted
// by the keyword `dyn` are also unsized. Trait
// objects are just pointers to a value implementing
// a certain trait.
//
// Rust can't work with unsized types directly.
// You have to introduce a level of indirection
// using pointers (smart pointers/references) when
// working with them. Pointers to unsized values
// are always *fat pointers* (two words size):
//
// * for slices it's the pointer and the length
// * for trait objects it's the pointer to the
//   object and the vpointer.
//
// For generics, Rust actually defaults to them
// being Sized implicitly. You have to explicitly
// declare them to optionally being unsized using
// the trait bound `?Sized` (questionably sized).
//
// There is another kind of unsized type: A struct's
// **last** field can be unsized. For example the
// `Rc<T>` is implemented internally as a pointer to
// the internal type `RcBox<T>`. Here's a simplified
// version:
struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T, // Here, `T` is questionably sized
}

// `RcBox<T>` can hold sized as well as unsized types `T`.
// You can't create one for an unsized `T` directly, yo
// first need to create a sized `RcBox<T>` and then convert
// the ref to a fat pointer:
fn main() {
    // (1) Create the sized box.
    let box_string: RcBox<String> = RcBox {
        ref_count: 1,
        value: "data".to_string(),
    };

    use std::fmt::Display;
    // (2) Convert the sized box ref to a fat pointer.
    let box_displayable: &RcBox<dyn Display> = &box_string;
}
