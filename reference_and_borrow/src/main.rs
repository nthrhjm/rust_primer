//参照と借用
//参照の規則
//任意のタイミングで、ひとつの可変参照or複数の参照を行える
//参照は常に有効でなければならない
fn main() {
    //値の所有権をもらう代わりに引数としてオブジェクトへの参照を取る関数
    //参照を渡すことで所有権を維持できる
    //関数のパラメーターに参照をとることを借用と呼ぶ。
    let s1 = String::from("hello"); //s1がスコープに入る
    let len = calculate_length(&s1); //関数calculate_length()はs1の参照(&s1)を借用する
    println!("The length of {} is {}", s1, len); //関数に渡したのは参照なのでs1を使用できる
                                                 //The length of hello is 5

    //可変な参照を用いて借用先の関数で値を変更する
    let mut s = String::from("hello"); //可変変数を定義
    change(&mut s); //可変な参照を渡す

    //可変な参照の制約
    //特定のスコープで、ある特定のデータに対しては、ひとつしか可変な参照を持つことはできない。
    //以下のコードはエラーとなる cannot borrow `s` as mutable more than once at a time
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("r1 is {}, r2 is {}", r1, r2);

    //スコープが異なると複数の可変な参照を持つことができる。
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1:{}", r1); //r1:hello
    }
    let r2 = &mut s;
    println!("r2:{}", r2); //r2:hello
}

fn calculate_length(s: &String) -> usize {
    //パラメーターにString型ではなくその参照（&String）を受け取る（借用）
    s.len()
} //ここでsがスコープから外れるが、sはStringへの参照で所有権を持っていない。よってなにも起こらない。

fn change(some_string: &mut String) {
    //可変な参照をパラメーターとして受け取る（借用する）
    some_string.push_str(", world.");
}
