fn main() {
    let speech = "\"Ouch!\" said the well.\n";

    // Strings can span multiple lines (line breaks become `\n`).
    println!(
        "foo
    bar"
    );

    // When escaping the line break we avoid the `\n`.
    println!(
        "foo \
    bar"
    );

    // Raw strings ignore all escape sequences.
    let raw_string = r###"C:\Program Files\Gorillas"###;

    // Byte strings aren't UTF-8 encoded, they are slices of u8 values.
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // `&str` are string slices - refences to UTF-8 encoded strings.
    // Like other slices, they are 2 machine word fat pointers (address and length).
    // String slices are just byte slices `&[u8]` guaranteed to hold well-formed
    // UTF-8 strings. String literals are actually of type `&str`.
    let str = "Hello, world!";

    // &str -> &[T]
    // String -> Vec<T>

    // Aside: It's possible to define type aliases in Rust.
    type Bytes = Vec<u8>;
}
