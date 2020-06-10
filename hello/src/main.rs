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
}
