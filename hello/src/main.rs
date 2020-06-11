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
}
