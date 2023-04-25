fn main() {
    // Checked operations return an Option of the result:
    // Some(v) if the mathematically correct result can be
    // represented as a value of the type, or None if it
    // cannot.
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!((-128_i8).checked_div(-1), None);


    // Wrapping operations return the value equivalent to the 
    // mathematically correct result modulo the range of the value
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance is wrapped
    // to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // Saturating opertions return the representable value that is
    // closest to the mathematically correct result
    assert_eq!(32760_i16.saturating_add(16), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // Overflowing operations return a tuple (result, overflowed), where
    // result is what the wrapping version of the function would return,
    // and overflowed is a bool indicating whether an overflow occurred
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    let text = "I see the eigenvalu in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
    println!("{}", text);
}
