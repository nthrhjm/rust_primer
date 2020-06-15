//構造体
fn main() {
    //Userのインスタンス生成
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //構造体のフィールドへのアクセス（ドット記法)
    println!("user1's email = {}", user1.email);
    //インスタンスが可変ならフィールドを書き換えられる
    user1.email = String::from("anotheremail@example.com");
    println!("changed user1's email = {}", user1.email);

    //関数を使った構造体の生成
    let user2 = build_user(
        String::from("secondemail@example.com"),
        String::from("someusername456"),
    );
    println!("user2 status");
    println!("\temail: {}", user2.email);
    println!("\tuser name: {}", user2.username);
    println!("\tactive: {}", user2.active);
    println!("\tsign in count: {}", user2.sign_in_count);

    //構造体交信記録法で他のインスタンスから新しくインスタンスを生成する
    let user3 = User {
        email: String::from("third@example.com"),
        username: String::from("thirdusername789"),
        ..user2 //上記２項目以外はuser1のデータを使う
    };
    println!("user3 status");
    println!("\temail: {}", user3.email);
    println!("\tuser name: {}", user3.username);
    println!("\tactive: {}", user3.active);
    println!("\tsign in count: {}", user3.sign_in_count);

    //タプル構造体
    //フィールドが型と値で構成される（名前がない）
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0); //Colorタプル構造体のインスタンス
    let origin = Point(0, 0, 0); //Pointタプル構造体のインスタンス
    println!("black R:{} G:{} B:{}", black.0, black.1, black.2);
    println!("origin x:{} y:{} z:{}", origin.0, origin.1, origin.2);
}
//Userを表す構造体の定義
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
//構造体Userを返す関数
fn build_user(email: String, username: String) -> User {
    User {
        email,    //フィールド名とパラメータが同名なのでセミコロン以下を省略できる
        username, //上と同様
        active: true,
        sign_in_count: 1,
    }
}
