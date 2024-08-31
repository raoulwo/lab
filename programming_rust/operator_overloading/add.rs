// In Rust, expressions like `a + b` are actually shorthands
// for `a.add(b)` since operators are implemented as methods
// which are part of traits. By implementing the traits on
// your types, you make the corresponding operators available.

use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

// Generic implementation to add all Complex numbers of L to
// Complex numbers of R with the constraint that L can be
// added to R. The resulting Complex number will be of the
// result type of L + R.
impl<L, R> Add<Complex<R>> for Complex<L>
where
    L: Add<R>,
{
    type Output = Complex<L::Output>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

fn main() {
    let c0 = Complex { re: 1, im: 0 };
    let c1 = Complex { re: 0, im: 1 };
    println!("{:?}", c0 + c1);

    let c2 = Complex {
        re: 1.5f32,
        im: -0.5f32,
    };
    let c3 = Complex {
        re: -1.5f32,
        im: 0.5f32,
    };
    println!("{:?}", c2 + c3);
}
