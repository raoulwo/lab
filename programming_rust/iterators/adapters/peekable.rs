// The `peekable` method transforms an iterator
// into a *peekable* iterator, an iterator that can
// *peek* at the next item without consuming it using
// the `peek` method which returns an `Option<&Item>`.
use std::iter::Peekable;

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = char>,
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n,
        }
        tokens.next();
    }
}

fn main() {
    let mut chars = "1234,5678,9012,3456,7890".chars().peekable();
    assert_eq!(parse_number(&mut chars), 1234);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 5678);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 9012);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 3456);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 7890);
    assert_eq!(chars.next(), None);
}
