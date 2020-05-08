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

    assert_eq!('*' as i32, 42); // char型から整数型への変換

    // 変換系のメソッドは結果を成功すればSome/失敗すればNoneで返される
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21); // 2つの文字列スライスをtupleで受け取る
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // タプルでは最後のカンマは有っても無くても等価として扱われる
    assert_eq!(("Brazil", 1985,), ("Brazil", 1985));

    let lazy_caterer: [i32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthoropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // 長さ10000のbool配列を確保し、中身の一部をfalseに反転させる
    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // 厳密には配列chaosはsortメソッドを持たないが、暗黙的に&mut [i32]スライスが作られ、スライスが持つsortメソッドが呼ばれる
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // ベクタはvec!マクロで作るかVec::newで作る
    let mut v1 = vec![2, 3, 5, 7];
    assert_eq!(v1.iter().fold(1, |a, b| a * b), 210);
    v1.push(11);
    v1.push(13);
    assert_eq!(v1.iter().fold(1, |a, b| a * b), 30030);

    let mut v2 = Vec::new();
    v2.push("step");
    v2.push("on");
    v2.push("no");
    v2.push("pets");
    assert_eq!(v2, vec!["step", "on", "no", "pets"]);

    // collectを呼んでベクタを作る場合は、どの型のコレクションを欲しいのか明示することが多い
    let v3: Vec<i32> = (0..5).collect();
    assert_eq!(v3, [0, 1, 2, 3, 4]);

    // 配列と同様に、暗黙的に&mut [&str]スライスを借用し、スライスが持つメソッドreverseが呼ばれる
    let mut v4 = vec!["a man", "a plan", "a canal", "panama"];
    v4.reverse();
    assert_eq!(v4, vec!["panama", "a canal", "a plan", "a man"]);

    // ライブラリでは要素数を指定したベクタを確保して使うことが多い
    let mut v5 = Vec::with_capacity(2);
    assert_eq!(v5.len(), 0);
    assert_eq!(v5.capacity(), 2);
    v5.push(1);
    v5.push(2);
    assert_eq!(v5.len(), 2);
    assert_eq!(v5.capacity(), 2);
    v5.push(3); // capacityがあっても自動で拡張されヒープ再確保される
    assert_eq!(v5.len(), 3);
    assert_eq!(v5.capacity(), 4);

    // ベクタからの取り出し
    let mut v6 = vec!["carmen", "miranda"];
    assert_eq!(v6.pop(), Some("miranda"));
    assert_eq!(v6.pop(), Some("carmen"));
    assert_eq!(v6.pop(), None); // 格納されている値がなくなった

    // バイト文字列
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']); // methodは&[u8; 3] つまり3バイトのchar配列への参照

    // Stringの生成
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    // 文字列strの評価
    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("    clean\n".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}
