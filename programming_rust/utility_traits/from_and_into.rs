// The traits `std::convert::From` and `std::convert::Into`
// are used for **consuming type conversions** as opposed
// to `AsRef` and `AsMut` which only *borrow* references.
// `From` and `Into` take ownership of a value, transform
// that value and then return ownership back of the transformed
// value.

// Here are the definitions of the traits:
trait MyInto<T>: Sized {
    // `Self` -> `T`
    fn into(self) -> T;
}

trait MyFrom<T>: Sized {
    // `T` -> `Self`
    fn from(other: T) -> Self;
}

// NOTE: Each type `T` automatically implements `From<T>`, `Into<T>`

// The use of `Into` is similar to how `AsRef` is generally used.
// You can make functions more flexible in the types of args they
// can consume (again, similar to function overloading in C++):

use std::convert::Into;
use std::net::Ipv4Addr;

fn ping<A>(address: A) -> std::io::Result<bool>
where
    // `A` must be convertable into `Ipv4Addr`
    A: Into<Ipv4Addr>,
{
    Ok(true)
}

// `From` is used in different scenarios. The method `from` serves
// as a generic constructor for producing instances of a type from
// values of another type. Both traits are for **infallible**
// conversions, for conversions that might fail use `TryFrom` and
// `TryInto` instead.

// NOTE: When you have an appropriate `From` implementation, the
// standard library automatically implements the corresponding
// `Into` for you!

// WARNING: `AsRef` and `AsMut` conversions are expected to be cheap,
// the conversions using `From` and `Into` don't have to be! E.g.
// `String` implements `From<&str>` which copies the string slice
// to a new heap-allocated buffer.

// NOTE: The `?` operator used for short-circuiting error handling
// in functions uses `From` and `Into` to automatically convert error
// types into more general ones when needed.

fn main() {}
