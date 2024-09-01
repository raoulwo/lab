// Normally, when you want to create a copy of
// a given value you use the trait `Clone`. This
// doesn't work for types like `str` or `[i32]`
// when you want to create a copied `String` or
// `Vec<i32>` out of those values. The definition
// of `Clone` specifies that `&T` must always
// return a `T` which doesn't work for unsized
// types. Instead, you can use `std:borrow::ToOwned`
// for a looser way of converting a ref to an owned
// value:
trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}

// Instead of returning `Self` like `Clone` does, we
// can return anything that we could borrow `&Self`
// from.

// NOTE: You can view `ToOwned` as the counterpart
// of `Borrow`:
//
// * You can borrow `&[T]` from `Vec<T>`, so `[T]`
//   can implement `ToOwned<Owned=Vec<T>>`.
// * You can borrow `&str` from `String`, so `str`
//   can implement `ToOwned<Owned=String>`.
