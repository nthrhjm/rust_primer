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
}
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
