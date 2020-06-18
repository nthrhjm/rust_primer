//文字列
//str(&str)...言語の核として存在する。別の場所に格納されたUTF-8エンコードされた文字列データへの参照となる文字列スライス。
//文字列リテラルは、プログラムのバイナリ出力に格納されるので文字列スライスになる。
//String型...言語の核としてではなくRustの標準ライブラリで提供される。
//伸長可能、可変、所有権のあるUTF-8エンコードされた文字列型。
//Rustにおいて文字列を指すのはどちらかではなくString型と文字列スライスの&strのことをいう。
fn main() {
    //新規文字列を生成する
    //新しい空のsという可変文字列を生成する
    let mut s = String::new();

    //文字列の初期値を使う
    let data = "initial contents";
    let s = data.to_string();
    println!("s = {}", s); //s = initial contents
                           //こちらの記法でも良い？
    let s = "initial contents".to_string();
    println!("s = {}", s); //s = initial contents

    //String::from関数を使っても良い
    //String::from関数とto_stringメソッドは全く同じことをしている
    let s = String::from("initial contents with String::from");
    println!("s = {}", s); //s = initial contents with String::from

    //文字列を更新する
    //push_strで文字列に追加する
    let mut s = String::from("foo");
    s.push_str("bar"); //push_strは所有権を取らない
    println!("{}", s); //foobar

    let mut s1 = String::from("foo");
    let s2 = "bar"; //文字列リテラル
    s1.push_str(s2); //push_strがs2の所有権を奪わないので...
    println!("s2 is {}", s2); //s2 is bar (この行でs2を読むことができる)

    //pushメソッドは1文字を取りStringに追加する。
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s); //s is lol

    //+演算子やformat!マクロで連結
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1はムーブされもう使えないことに注意
    println!("s3 is {}", s3); //s3 is Hello, world!

    //format!マクロはprintln!マクロと同様な動きをするが、結果を標準出力に出力する代わりにStringで返す。
    //+演算子を使うよりも読みやすく、引数の所有権を奪わない。
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s); //s is tic-tac-toe

    //文字列に添字アクセスする
    //以下のコードはエラーを吐く
    let s1 = String::from("hello");
    //let h = s1[0];
    //エラーメッセージ： the type `std::string::String` cannot be indexed by `{integer}`
    //RustではStringに対して添字アクセスはできない

    let len = String::from("Hola").len();
    println!("Hola の長さは {}", len); //Hola の長さは 4
    let len = String::from("Здравствуйте").len();
    println!("Здравствуйте の長さは {}", len); //Здравствуйте の長さは 24  （つまり1文字2バイト)
                                               //つまり見た目の文字数とは異なる

    //文字列をスライスする
    //[]で一つの数値により添字アクセスするのではなく、
    //範囲とともに[]を使って、特定のバイトを含む文字列スライスを作る
    let hello = "Здравствуйте";
    let s = &hello[0..4]; //sは最初の4バイトを含む&strになる。つまり`Зд`
    println!("s is {}", s); //s is Зд

    //次のコードは実行時パニックになる
    //let s = &hello[0..1];
    //thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
    //バイト添字1は文字の境界ではありません;`Здравствуйте`の'З'(バイト番号0から2)の中です」でパニックしました)

    //文字列を走査するメソッド群
    //charsメソッド
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    //न
    //म
    //स
    //
    //त
    //

    //bytesメソッドは各バイトをそのまま返す
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // 224
    // 164
    // 168
    // 224
    // 164
    // 174
    // 224
    // 164
    // 184
    // 224
    // 165
    // 141
    // 224
    // 164
    // 164
    // 224
    // 165
    // 135

    for c in "おはよう".chars() {
        println!("{}", c);
    }
    //お
    //は
    //よ
    //う
}
