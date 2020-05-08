fn main() {
    println!("Hello, world!");
}

#[test]
fn test_rust_type() {
    assert_eq!(   10_i8  as u16,    10_u16); // u16の範囲内
    assert_eq!( 2525_u16 as i16,  2525_i16); // i16の範囲内
    assert_eq!(   -1_i16 as i32,    -1_i32); // 符号拡張
    assert_eq!(65535_u16 as i32, 65535_i32); // ゼロ拡張
}
