fn main() {
    // Overflowing arithmetic operations cause a panic in debug builds.
    // In release builds, the integer values would wrap instead.

    // Checked operations will return an Option<T>.
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!((-128_i8).checked_div(-1), None);

    // Wrapping operations return the resulting value modulo the range of the value.
    assert_eq!(100_u16.wrapping_mul(200), 20_000);
    assert_eq!(500_u16.wrapping_mul(500), 53_392);
    assert_eq!(500_i16.wrapping_mul(500), -12_144);

    // Saturating operations clamp the result to the max/min value.
    assert_eq!(32_760_i16.saturating_add(10), 32_767);
    assert_eq!((-32_760_i16).saturating_sub(10), -32_768);

    // Overflowing operations return a tuple (result, overflowed), where `result`
    // is the possibly wrapping value and `overflowed` is a bool indicating whether
    // an overflow occured.
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
}
