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

    //タプルのベクタに対してcollectメソッドを使用してHashMapを生成する
    let teams = vec![String::from("blue"), String::from("Yellow")]; //チーム名を持つベクタ
    let initial_scores = vec![10, 50]; //得点を持つベクタ

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    //zipメソッドでteamsベクタとinitial_scoresベクタを合わせたタプルのベクタを作る（例："Blue"と10のタプル,"Yellow"と50のタプル）
    //さらにcollectメソッドはタプルのベクタからhashmapを返す。

    //HashMapの所有権
    //i32のようなCopyトレイトを実装する型は値はHashMapにコピーされる
    //Stringのような所有権のある値なら値はmoveされHashMapはそれらの値の所有者になる
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); //insertメソッドでfield_name, field_valueをmapに(K, V)として挿入
                                         //ここでfield_name, field_valueは値の所有権を失い無効になる

    //println!("field_name is {}", field_name); //error[E0382]: borrow of moved value: `field_name`

    //HashMapの値にアクセスする
    //HashMapのgetメソッドにキーを渡すことでHashMapから値を取り出すことができる
    let mut scores = HashMap::new(); //空のHashMapを生成
    scores.insert(String::from("Blue"), 10); //値を挿入
    scores.insert(String::from("Yellow"), 50); //値を挿入

    let score = scores.get("Blue"); //getメソッドは値をOption<&V>型として返す
                                    //なぜならgetメソッドは与えられたキーに対応する値が
                                    //HashMapに存在しない場合にNoneを返す場合があるため

    //Option<&V>型であるsocreの値を見るにはmatch式かif let式を使う
    if let Some(i) = score {
        println!("Blue teams score is {}", i); //Blue teams score is 10
    };

    //ベクタの様にfor loopでHashMapのキーと値を走査することができる
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    //Yellow: 50
    //Blue: 10

    //HashMapを更新する
    //値を上書きする(insertメソッド)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); //下のコードで10という値は上書きされる
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); //{"Blue": 25}

    //キーに値がなかった時のみ値を挿入する(entryメソッド)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    //or_insert(値)メソッドはentry(キー)が存在しなかった場合そのキーに対応する値を挿入する
    scores.entry(String::from("Yellow")).or_insert(50); //scoresは"Yellow"キーを持たないので"Yellow": 50という組を挿入
    scores.entry(String::from("Blue")).or_insert(50); //scoresは"Blue"キーを持つのでこの行は無効
    println!("{:?}", scores); //{"Yellow": 50, "Blue": 10}

    //古い値に基づいて値を更新する
    let text = "hello world wonderful world";
    let mut map = HashMap::new(); //空のHashMapを生成
    for word in text.split_whitespace() {
        //textをホワイトスペース区切りにしたものをwordとしてひとつずつ取り出し
        let count = map.entry(word).or_insert(0); //mapにwordキーがなければその値として0をセット
                                                  //or_insertメソッドはキーに対する値への可変参照(&mut V)を返す
        *count += 1; //countは可変参照(&mut V)なので変更するために`*`で参照外しが必要
    }
    println!("{:?}", map) //{"hello": 1, "world": 2, "wonderful": 1}
}
