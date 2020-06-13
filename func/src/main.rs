fn main() {
    no_param();
    one_param(10);
    two_param(10, 20);
    let ret = two_param_and_return(10, 20);
    println!("ret is {}", ret);
    let mut s = String::new();
    str_param_complex(&mut s); //変更可能な参照を渡す &mut
    println!("s is {}", s);
    str_param("Rust");

    let ret = str_param_and_return("Rust");
    println!("ret is {}", ret);

    let v = vec![1, 2, 3, 4, 5];
    let sum = vec_param(&v);
    println!("sum is {}", sum);

    let v = vec_return(10);
    for i in v {
        print!("{} ", i);
    }
    println!("");

    let mut v = vec![1, 2, 3, 4, 5];
    vec_change(&mut v);
    for i in v {
        print!("{} ", i);
    }
    println!("");
}
//----------------ここまでmain()関数--------------------------
//引数も戻り値もない関数
fn no_param() {
    println!("called no_param");
}
//引数を持つ関数
fn one_param(x: i32) {
    println!("called one_param, x is {}", x);
}
fn two_param(x: i32, y: i32) {
    println!("called two_param, {} and {}", x, y);
}
//戻り値を持つ関数
fn two_param_and_return(x: i32, y: i32) -> i32 {
    println!("called two_param_and_return, {} and {}", x, y);
    x + y //セミコロンをつけないと式となり関数の返り値になる
}
//文字列を受け取る関数
fn str_param(s: &str) {
    //引数の型を&strとする &は参照するという意味
    println!("called str_param, s is {}", s);
}
//パラメーターsとして変更可能な文字列の参照を受け取る
fn str_param_complex(s: &mut String) {
    *s = String::from("hello"); //helloという文字列を生成してパラメーターsが参照する値に代入
}
//固定文字列の参照を受け取りStringを返す関数
fn str_param_and_return(s: &str) -> String {
    //参照ではなくStringの実体そのものを返す（&Stringではない）
    println!("called str_param_and_return, s is {}", s);
    let ret = format!("hello {} world.", s);
    ret
}
//ベクターを受け取る関数
fn vec_param(v: &Vec<i32>) -> i32 {
    println!("called vec_param");
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

//ベクターを返す関数
fn vec_return(max: i32) -> Vec<i32> {
    println!("called vec_return");
    let mut v = Vec::new(); //変更可能な空のベクターを生成してvに代入
    for i in 0..max {
        //0..10 で0から9(10 - 1)までの範囲をあらわす
        v.push(i); //vにiを順にpushしていく
    }
    v
}

//ベクターの中身を変更する関数
fn vec_change(v: &mut Vec<i32>) {
    println!("called vec_change");
    for i in v {
        *i = *i * 10;
    }
}
