fn main() {
    let a = 10;
    let b = 20;
    if a == b {
        //if文の条件式に()は不要。()を付けるとwarningがでる
        println!("a == b is ok.");
    } else if a < b {
        println!("a < b is ok.");
    } else {
        println!("a > b is ok.");
    }

    //論理積
    if a == 10 && b == 20 {
        println!("AND is ok.");
    }
    //論理和
    if a == 0 || b == 20 {
        println!("OR is ok");
    }

    //関数の戻り値を直接参照する
    let a = 10;
    let b = 20;
    if test(a, b) {
        //bool値を返すtest()関数の呼び出しを条件式とする
        println!("test is ok.");
    }

    //if文で値を返す
    let a = 10;
    let b = 20;
    let x = if a < b { 1 } else { 0 }; //ifブロックとelseブロックで返す値の型は同じものでなければならない
    println!("x is {}", x);

    //繰り返し
    //ベクターを使って繰り返し処理をする
    let v = vec![10, 20, 30, 40, 50];
    print!("v is ");
    for i in &v {
        print!("{} ", i);
    }
    println!("");
    //v is 10 20 30 40 50

    //for文でイテレーターを利用する
    let v = vec![10, 20, 30, 40, 50];
    print!("v is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");
    //v is 10 20 30 40 50

    //for文をインデックス付きで繰り返す
    let v = vec![10, 20, 30, 40, 50];
    print!("v is ");
    for (i, x) in v.iter().enumerate() {
        //イテレータにenumerateを実行すると,添字と要素の参照をタプルで得ることができる
        print!("{}:{} ", i, x);
    }
    println!("");
    //v is 0:10 1:20 2:30 3:40 4:50

    //for文で繰り返し数を指定する
    //範囲の指定は n..m(nからm-1までの範囲)
    print!("FOR is ");
    for i in 0..10 {
        //0..10で0から9までの範囲を表す
        print!("{} ", i);
    }
    println!("");
    //FOR is 0 1 2 3 4 5 6 7 8 9

    //繰り返しを途中で止める(break文)
    print!("FOR is ");
    for i in 0..10 {
        if i == 5 {
            break;
        }
        print!("{} ", i);
    }
    println!("");
    //FOR is 0 1 2 3 4

    //繰り返しの途中で先頭に戻る(continue文)
    print!("FOR is ");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!("");
    //FOR is 1 3 5 7 9

    //while文で継続条件を指定する
    print!("WHILE is ");
    let mut i = 0;
    while i < 10 {
        print!("{} ", i);
        i += 2;
    }
    println!("");
    //WHILE is 0 2 4 6 8

    //loop文で無限ループ
    print!("LOOP is ");
    let mut i = 0;
    loop {
        //無限ループ
        if i >= 10 {
            break; //無限ループを抜ける
        }
        print!("{} ", i);
        i += 1;
    }
    println!("");
    //LOOP is 0 1 2 3 4 5 6 7 8 9
}

fn test(x: i32, y: i32) -> bool {
    x < y
}
