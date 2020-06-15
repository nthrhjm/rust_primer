fn main() {
    let mut s = String::from("hello"); //String型の値"hello"はヒープに置かれる
                                       //スタック上に置かれた変数sは束縛された値"hello"のメモリへのポインタと長さなどを持つ
    s.push_str(", world"); //String型の値は内容を変更できる
    println!("{}", s); //hello, world

    //moveとcopy
    //スカラー値の代入はcopy
    let x = 5; //スカラー値はスタックに保存される
    let y = x; //変数の代入はcopyになるので代入後もxは使用できる
    println!("x:{}, y{}", x, y); //x:5, y:5

    //文字列の代入はmove
    // let s1 = String::from("hello"); //String型の文字列"hello"を生成してs1に束縛
    // let s2 = s1; //s1の値をs2にmove(s1は無効化されるので使用できない)
    // println!("s1:{}, s2:{}", s1, s2); //無効化されたs1を呼び出したのでエラーになる

    //文字列の値（ヒープ上）ごと変数の値（スタック上）をコピーしたい場合
    let s1 = String::from("hello");
    let s2 = s1.clone(); //clone()メソッドはヒープデータのdeep copyを行う
    println!("s1:{}, s2:{}", s1, s2); //s1:hello, s2:hello

    //所有権と関数
    //関数に値を渡すことと、値を変数に代入することは似ている。関数に変数を渡すと、代入のようにmoveやcopyが発生する。
    let s = String::from("hello"); //sがスコープに入る
    takes_ownership(s); //sの値が関数takes_ownership()にmoveされて
                        //ここからはsは有効ではなくなる
    let x = 5; //xがスコープに入る
    makes_copy(x); //xが関数makes_copy()に渡されるがi32型はCopyなので、
                   //この後（main()スコープ内なら）にxを使用しても大丈夫

    //戻り値とスコープ
    //値を返すことでも所有権は移動する
    let s1 = gives_ownership(); //gives_ownership()は戻り値をs1にmoveする
    let s2 = String::from("hello"); //s2がスコープに入る
    let s3 = takes_and_gives_back(s2); //s2はtakes_and_gives_back()にmoveされ、takes_and_gives_back()の戻り値もs3にmoveされる
} //ここでs3がスコープから抜け、ドロップされる。s2もスコープを抜けるが関数にmoveされているので何も起きない。
  //s1もスコープを抜け、ドロップされる。

fn takes_ownership(some_string: String) {
    //some_stringがスコープに入る
    println!("{}", some_string);
} //ここでsome_stringがスコープを抜け、`drop`が呼ばれる。メモリが解放される。
fn makes_copy(some_integer: i32) {
    //some_integerがスコープに入る
    println!("{}", some_integer);
} //ここでsome_integerがスコープを抜ける。何も特別なことは起こらない。

fn gives_ownership() -> String {
    let some_string = String::from("hello"); //some_stringがスコープに入る
    some_string //some_stringが返され、呼び出し元（main関数)にmoveされる
}

fn takes_and_gives_back(a_string: String) -> String {
    //a_stringがスコープに入る
    a_string //a_stringが返され、呼び出し元（main関数)にmoveされる
}
