trait Foo {
    fn foo(&self) -> String;
}

// The trait `Bar` is a *subtrait* of `Foo`, `Foo` is the
// *supertrait* of `Bar`. This means all types implementing
// `Bar` **must** implement `Foo` as well.
trait Bar: Foo {
    fn bar(&self) -> String;
}

// NOTE: In Rust, subtraits **don't** inherit the items of its
// supertrait. You still have to put the supertrait in scope to
// use its methods...

// NOTE: Subtraits are just a shorthand for a bound on the `Self`
// type, `Bar` could also be defined as: `trait Bar where Self: Foo {...}`

fn main() {}
