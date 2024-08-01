fn main() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(&v);
    print(&a);

    print(&v[1..]);
    print(&v[..2]);
    print(&v[1..=2]);
}

fn print(n: &[f64]) {
    for elt in n {
        print!("{} ", elt);
    }
    print!("\n");
}
