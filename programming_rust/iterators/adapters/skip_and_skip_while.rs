// `skip` and `skip_while` are the counterparts of
// `take` and `take_while`. Instead of taking the first `n`
// values or producing values while a predicate returns true,
// they drop the first `n` values or drop items as long as the
// predicate is true.

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for num in v.iter().skip(3) {
        println!("{}", num);
    }

    let text = "# Heading 1\n
                \n
                This is a paragraph.\n";

    for body in text
        .lines()
        .map(str::trim)
        .skip_while(|line| !line.is_empty())
        .skip(2)
    {
        println!("{}", body);
    }
}
