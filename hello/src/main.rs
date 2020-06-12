fn main() {
    println!("Hello, world!");

    //変数宣言の型アノテーションは初期値があれば省略できる（型推論）
    let name: &str = "hajime nutahara";
    let age: i32 = 18;
    println!("My name is {}. I am {} years old.", name, age);

    //関数のパラメータと返り値の型アノテーションは必須
    fn add(x: i32, y: i32) -> i32 {
        //i32型のパラメータを二つ受け取り、i32型の値を返す
        return x + y;
    }
    let a = 12;
    let b = 24;

    println!("{} + {} = {}", a, b, add(a, b));

    //Rustの浮動小数点はf32とf64があり、型アノテーションを省略するとf64として扱われる
    let x = 100.234;
    println!("x is {}", x);
    let x: f64 = 100.234;
    println!("x is {}", x);

    //真偽値の型はbool
    let boole: bool = true;
    println!("boole is {}", boole);

    //文字型(char)
    let c = 'あ';
    let dog = '🐶';
    let cat: char = '🐱';
    println!("{} {} {}", c, dog, cat);

    //文字列(&str)
    let dog = "DOG";
    let cat = "CAT";
    println!("{} and {}", dog, cat);

    //文字列(String)
    let a = String::from("Hello Rust world.");
    println!("{}", a);

    //Stringは文字連結(+)ができる
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("world.");
    let s = s1 + " " + &s2 + " " + &s3; //2文字目以降は&をつける（借用）
    println!("{}", s);

    //format!マクロを使った文字列連結
    let s4 = String::from("Hello");
    let s5 = String::from("Rust");
    let s6 = String::from("world.");
    let s = format!("{} {} {}", s4, s5, s6);
    println!("{}", s);

    //タプル型(複数の異なる型をまとめて扱う)
    let t = ("nutahara", 18);
    println!("name is {} age {}", t.0, t.1);

    //配列型(後から要素を書き換えたり長さを変えることはできない)
    let a = ["春", "夏", "秋", "冬"];
    println!("最初の季節 {}", a[0]);
    println!("最後の季節 {}", a[3]);

    //参照と借用
    //所有権と移動
    //数値の場合
    let x = 100;
    let y = x; //xが指し示す値がyにコピーされるので以下のprintln!マクロは両方とも実行できる。
    println!("x is {}", x);
    println!("y is {}", y);

    //文字列の場合
    let m = String::from("Hello");
    let n = m; //mの中身がnにmoveされる。mは空になる。mの所有権がnに移動（move）したという。
               //所有権とは固定メモリを示すことができる権限。
               //Rustでは一つの固定メモリを指し示す変数は一つのみというルールが存在する。
               //println!("m is {}", m); //空を参照したのでコンパイルエラーが発生する。
    println!("n is {}", n); //nの中身は"Hello"なので正常に表示できる。

    //借用(borrow)
    //引数がstring型の関数を使用した場合
    let x = String::from("Hello");
    let len = string_length(x); //xの示す値をstring_length()関数にmoveしている
    println!("len is {}", len);
    //println!("x is {}", x); //string_length()関数に値をmoveした後なのでコンパイルエラーになる

    //引数が参照(reference)型の関数を使用した場合
    let x = String::from("Bye");
    let len = ref_string_length(&x); //xの参照（&x）を渡すことで所有権を手放さない
    println!("len is {}", len);
    println!("x is {}", x); //値"Bye"の所有権をxが保持したままなのでxを参照したprintln!マクロを実行できる

    //束縛
    let x = 100;
    let mut y = 200;
    println!("x is {}", x);
    println!("y is {}", y);
    //x = 200; let で宣言した変数に値を代入するのはダメ
    y = 400; //let mut 宣言した変数は再代入おk
    println!("x is {}", x);
    println!("y is {}", y);

    //スコープ
    println!("===scope===");
    let x = 100;
    println!("x is {}", x);
    {
        let x = 200;
        println!("x in block is {}", x);
    }
    println!("x is {}", x);

    println!("{}", test(99));
    println!("{}", test2(99));
    let ans = add_two(10, 20);
    println!("ans is {}", ans);
    let ans = add_one(30);
    println!("ans is {}", ans);

    //構造体
    struct Sample {
        x: i32,
    }
    impl Sample {
        fn new(x: i32) -> Sample {
            Sample { x: x }
        }
        fn inc(&self) -> i32 {
            self.x + 1
        }
        fn add(&self, x: i32) -> i32 {
            self.x + x
        }
    }

    let a = Sample::new(10);
    let ans = a.inc();
    println!("ans is {}", ans);
    let ans = a.add(20);
    println!("ans is {}", ans);

    //クロージャ
    let num = 10;
    let add_one = |x| num + x;
    let add_two = |x, y| x + y;

    let ans = add_one(1);
    println!("ans is {}", ans);
    let ans = add_two(10, 20);
    println!("ans is {}", ans);

    //文字(char型),Rustの1文字は32ビット(4バイト)
    let ch = 'A';
    println!("ch is {}", ch);
    let ch = 'あ';
    println!("ch is {}", ch);
    let ch = '🐈';
    println!("ch is {}", ch);
    let ch = '💎';
    println!("ch is {}\n", ch);
    let ch = '\u{1F431}';
    println!("{}\n", ch);

    let ch = 'A';
    println!("ch is {}", ch); // ch is A
    let u = ch as u8; //as u8 でcharからu8にキャスト
    println!("u is {}", u); // u is 65
    let ch = u as char; //as char でu8からcharにキャスト
    println!("ch is {}\n", ch); // ch is A
                                //ASCII文字の操作
    let s = "hello rust world.";
    println!("s is {}", s); //s is hello rust world.
    let hello = &s[0..5]; //文字列スライス 0文字目〜5文字目の前（4文字目）
    let world = &s[11..]; //文字列スライス 11文字目〜最後の文字まで
    println!("hello is {}", hello); //hello is hello
    println!("world is {}", world); //world is world. let len = s.len();
    let len = s.len(); //&str.len()関数は文字列の長さを返す
    println!("s.len is {}\n", len); //s.len is 17
    let mut s = String::new(); //String::new() 空文字を生成
    s.push_str("hello ");
    s.push_str("rust ");
    s.push_str("world.");
    println!("s is {}\n", s); //s is hello rust world.
    let hello = "HELLO";
    let rust = "RUST";
    let world = "WORLD.";
    let s = format!("{} {} {}", hello, rust, world);
    println!("s is {}", s); // s is HELLO RUST WORLD.

    //&strは固定文字列、&Stringは可変文字列
    //&String型の文字列を生成する
    let s = "hello rust world.".to_string();
    println!("s is {}", s); //s is hello rust world.
    let s = String::from("hello rust world.");
    println!("s is {}\n", s); //s is hello rust world.
                              //日本語の扱い
    let s = "こんにちは rust コードの世界";
    println!("s is {}\n", s); //s is こんにちは rust コードの世界

    //実行エラーになる例
    //let hello = &s[0..5]; //日本語は1文字3バイトなので[0..5]の文字区切りが2文字目の途中になるので実行時エラーになる
    //let world = &s[11..]; //同様に文字区切りが文字の途中にくるので実行時エラーになってしまう
    //日本語の区切りに合わせた例
    let hello = &s[0..15];
    let world = &s[21..];
    println!("こんにちは is {}", hello);
    println!("コードの世界 is {}", world);
    //日本語文字列の長さを取得
    let len = s.len();
    println!("s.len is {}", len); //s.len is 39

    //日本語文字列の連結
    let mut s = String::new();
    s.push_str("こんにちは ");
    s.push_str("rust ");
    s.push_str("コードの世界");
    println!("s is {}\n", s); //s is こんにちは rust コードの世界

    //format!マクロによる日本語文字列連結
    let hello = "こんにちは";
    let rust = "RUST";
    let world = "コードの世界";
    let s = format!("{} {} {}", hello, rust, world);
    println!("s is {}", s); //s is こんにちは RUST コードの世界

    //日本語文字列の&strから&Stringへの変換
    let s = "こんにちは Rust コードの世界".to_string();
    println!("s is {}", s);
    let s = String::from("こんにちは rust コードの世界");
    println!("s is {}\n", s);

    //文字列からchar型のベクター型を生成する
    let s = "This is ねこ😸neko 文字列";
    let mut v: Vec<char> = Vec::new(); //char型の空のベクターを生成
    for c in s.chars() {
        v.push(c); //文字列sから1文字ずつ取り出しchar型ベクターvの末尾にpushしていく
    }
    let v = &v[8..15]; //vの8文字目から14文字目までを取得
    let mut s = String::new(); //空文字列を生成
    for c in v {
        s.push(*c) //char型ベクターvの要素を空文字列sにpushしていく
    }
    println!("s is {}", s); //s is ねこ😸neko

    //文字列スライス
    let s = "hello rust world.";
    let a = &s[0..1];
    println!("first is {}", a);
    let a = &s[0..5];
    println!("fist to five is {}", a);
    let a = &s[..5];
    println!("first to five is {}", a);

    //式と文
    //末尾に;をつけると文になる
    //値を返すものが式になる
    //{ 10 + 20} は式
    //{ 10 + 20;} は文

    //ビット演算子(& |)
    let a: u8 = 0b1111;
    let b: u8 = 0b0011;
    println!("a & b is {:04b}", a & b); //a & b is 0011
    println!("a | b is {:04b}", a | b); //a | b is 1111

    //シフト演算子(<< >>)
    let a: u8 = 0x02;
    println!("a << 1 is {}", a << 1); //a << 1 is 4
    println!("a >> 1 is {}", a >> 1); //a >> 1 is 1

    //論理演算子(&& ||)
    let a = true;
    let b = false;
    println!("a && b is {}", a && b); //a && b is false
    println!("a || b is {}", a || b); //a || b is true
}
//-----------------------------------------------------------------------------------
//引数はSting型
fn string_length(s: String) -> usize {
    let length = s.len();
    length
}
//引数は&String型
fn ref_string_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn test(x: i32) -> i32 {
    if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
        x
    }
}

fn test2(x: i32) -> i32 {
    let ans = if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
        x
    };
    ans
}

fn add_two(x: i32, y: i32) -> i32 {
    x + y
}
fn add_one(x: i32) -> i32 {
    x + 1
}
