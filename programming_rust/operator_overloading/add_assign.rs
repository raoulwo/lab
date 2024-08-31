use std::ops::AddAssign;

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<L, R> AddAssign<Complex<R>> for Complex<L>
where
    L: AddAssign<R>,
{
    fn add_assign(&mut self, rhs: Complex<R>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

fn main() {
    let mut c0 = Complex { re: -4, im: 5 };
    let c1 = Complex { re: 2, im: 3 };
    println!("{:?}", c0);

    c0 += c1;
    println!("{:?}", c0);
}
