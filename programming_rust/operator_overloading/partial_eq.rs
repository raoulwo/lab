use std::cmp::PartialEq;

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

fn main() {
    let c0 = Complex { re: 0, im: 0 };
    let c1 = Complex { re: 0, im: 0 };
    let c2 = Complex { re: 1, im: 0 };
    println!("{}", c0 == c1);
    println!("{}", c0 != c2);
}

// NOTE: It's called partial equality because
// for mathematical equivalence three properties
// are required:
//
// * x == x
// * x == y => y == x
// * x == y && y == z => x == z
//
// But with IEEE 754 floats some operations can
// result in NaN, and NaN == NaN is always false.
// So for partial equality the constraint x == x
// doesn't necessarily need to hold.
//
// For strict equality use std::cmp::Eq. Eq extends
// PartialEq, you can only implement Eq on types that
// are PartialEq as well.

// NOTE: In the standard lib f32, f64 are the only
// types that are partially equal but not strictly
// equal.
