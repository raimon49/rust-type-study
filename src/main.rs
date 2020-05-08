fn main() {
    println!("Hello, world!");
}

#[test]
fn test_rust_type() {
    assert_eq!(   10_i8  as u16,    10_u16); // u16の範囲内
    assert_eq!( 2525_u16 as i16,  2525_i16); // i16の範囲内
    assert_eq!(   -1_i16 as i32,    -1_i32); // 符号拡張
    assert_eq!(65535_u16 as i32, 65535_i32); // ゼロ拡張

    // as変換先の型範囲からはみ出すビットの切り捨て(truncation)
    assert_eq!( 1000_i16 as  u8, 232_u8);
    assert_eq!(65535_u32 as i16,  -1_i16);
    assert_eq!(   -1_i8  as  u8, 255_u8);
    assert_eq!(  255_u8  as  i8,  -1_i8);

    assert_eq!(2u16.pow(4), 16);             // 指数関数
    assert_eq!((-4i32).abs(), 4);            // 絶対値 -4i32.abs() と書くとメソッド呼び出しが負の値でなく正の値に呼ばれてしまう
    assert_eq!(0b101101u8.count_ones(), 4);  // ビットカウント

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // IEEE規格ではちょうど5になる
    assert_eq!((-1.01f64).floor(), -2.0);
    assert!((-1. / std::f32::INFINITY).is_sign_negative());

    // bool -> 整数への変換は可能だが、整数 -> boolへの変換はできない
    // Rustではbool型の情報量は1ビットだが、bool値のポインタを作れるようにするため1バイトまるまる使う
    assert_eq!(false as i32, 0);
    assert_eq!(true  as i32, 1);
}
