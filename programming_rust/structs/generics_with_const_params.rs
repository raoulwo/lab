// Generic structs can take constant params (integers, chars, bools are allowed)
struct Polynomial<const N: usize> {
    coefficients: [f64; N],
}

// The constant param can be used in type-associated functions
impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Polynomial { coefficients }
    }

    // ...
}

fn main() {
    // Rust can often infer the correct value for the const params.
    // Array with length 6 -> Polynomial<6>
    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
}
