//ベクタで一連の値を保持する

fn main() {
    //新しいベクタを作るにはVec::new関数を呼ぶ
    //新しい空のベクタを生成してi32型の値を保持する
    //空のベクタを作る時は型注釈が必須（どんな型が入るベクタかコンパイラが判断できないため）
    let v: Vec<i32> = Vec::new();

    //vec!マクロを使って初期値を持つベクタを生成する
    //初期値がある場合型注釈は不要
    let v = vec![1, 2, 3];

    //ベクタを更新する
    //pushメソッドを使用する
    let mut v = Vec::new(); //可変ベクタインスタンスを生成
                            //vにpushされるのはi32なので型注釈(Vec<i32>)がなくてもコンパイラが推論してくれる
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //ベクタの要素を読む
    let v = vec![1, 2, 3, 4, 5];
    //(取得方方その1)
    //添字記法で要素の参照を得る
    let third: &i32 = &v[2];
    println!("v's third data is {}", third);
    //(取得方法その2)
    //番号をgetメソッドに渡して、Option<T>を得る
    let third: Option<&i32> = v.get(2);
    match third {
        Some(i) => println!("v's third data is {}", i),
        None => (),
    }
    //二つの方法の違いはベクタの要素外の番号を指定した時にある
    //その1ではベクタの範囲外の値を指定するとパニックを起こしプログラムを終了させる
    //その2では範囲外の要素番号を指定してもパニックを起こさずNoneを返すのみ

    //ベクタの各値の参照を走査する
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    } //100
      //32
      //57

    //ベクタの各値の参照を変更を加える目的で走査する
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; //可変参照の値を変更するには参照外し演算子`*`を使っておかなければならない
        println!("{}", i);
    } //150
      //82
      //107

    //enumを使って複数の型を保持する
    enum SpreadssheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadssheetCell::Int(3),
        SpreadssheetCell::Text(String::from("blue")),
        SpreadssheetCell::Float(10.12),
    ];
}
