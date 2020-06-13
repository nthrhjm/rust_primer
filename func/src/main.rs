fn main() {
    no_param();
    one_param(10);
    two_param(10, 20);
    let ret = two_param_and_return(10, 20);
    println!("ret is {}", ret);
}
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
