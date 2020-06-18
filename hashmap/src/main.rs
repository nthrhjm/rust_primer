//ハッシュマップ
//キーと値のセットで値を保持する。
//型HashMap<K, V> K型のキーとV型の値
//キーの型Kは全て同じ型でなければならない
//値の型Vも全て同じ型でなければならない
//Rubyにおけるhash, JavaScriptにおけるObjectにあたる

use std::collections::HashMap; //use しないと使えない

fn main() {
    let mut scores = HashMap::new(); //空のHashMapを生成
    scores.insert(String::from("Blue"), 10); //Blueチームは10点
    scores.insert(String::from("Yellow"), 50); //Yellowチームは50点
}
