// Unlike slices, strings, vectors... iterators can't be
// compared using comparison/equality operators. They
// support comparison methods like `eq`, `lt`, `gt`...
// however.
fn main() {
    let str1 = "foo bar baz";
    let str2 = "foo   bar   baz";

    let str1_iter = str1.split_whitespace();
    let str2_iter = str2.split_whitespace();
    println!("str1_iter.eq(str2_iter): {}", str1_iter.eq(str2_iter));
}
