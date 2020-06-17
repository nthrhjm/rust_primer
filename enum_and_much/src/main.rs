//IPアドレス(v4 or v6)を表すenum
enum IpAddrKind {
    V4,
    V6,
}
//列挙子に値（型）を関連づけることもできる
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },    //匿名構造体を含む
    Write(String),              //単独のStringオブジェクトを含む
    ChangeColor(i32, i32, i32), //3つのi32値を含む
}
//enumにメソッドを定義
impl Message {
    fn call(&self) {
        //メソッド本体
    }
}

fn main() {
    //IpAddrKindの各列挙子のインスタンス化
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    //enum Option
    //Rustはnullを許容しない
    //その代わり、値が存在しているか不在かを表すenum Option<T>が存在する
    //標準ライブラリに定義されているOption<T>
    // enum Option<T>{
    //     Some(T),
    //     None,
    // }
    //Option<T>はスコープに導入しなくても使用でき、さらに列挙子のSome,Noneも
    //接頭辞Option::なしに直接使用できる
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    //Noneを使った場合、Option<T>の型を指定しなければならない
    //Noneだけでは何の型かコンパイラが推測できないため

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y; //この文はエラーになる
    //i8とOption<i8>は足し算できない
    //Option<T>型は使用する前にT型に変換する必要がある
    //     error[E0277]: cannot add `std::option::Option<i8>` to `i8`
    //   --> src/main.rs:53:17
    //    |
    // 53 |     let sum = x + y;
    //    |                 ^ no implementation for `i8 + std::option::Option<i8>`
    //    |
    //    = help: the trait `std::ops::Add<std::option::Option<i8>>` is not implemented for `i8`
}
