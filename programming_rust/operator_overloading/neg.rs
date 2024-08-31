use std::ops::Neg;

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T: Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

fn main() {
    let c = Complex { re: 2.0, im: 3.0 };
    println!("{:?}", -c);
}
