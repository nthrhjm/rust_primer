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
}

fn test(x: i32, y: i32) -> bool {
    x < y
}
