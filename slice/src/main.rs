//文字列スライス
fn main() {
    let s = String::from("Hello, world!");
    let hello = &s[0..5]; //"Hello"
    let world = &s[7..12]; //"world"
    let slice1 = &s[0..2]; //"He"
    let slice2 = &s[..2]; //"He" 始点を省略すると0として扱われる。上のコードと同義
    println!("{} {}", hello, world);
    println!("slice1 = {}, slice2 = {}", slice1, slice2);
    let word = first_word(&s);
    println!("first word is {}", word);

    let s = "Hello, world!"; //sの型は&str(不変な参照)
    let first = first_word(&s);
    println!("first word is {}(文字列リテラル)", first);
    let first_slice = first_word(&s[..]);
    println!("first word is {}(文字列リテラルのスライス)", first_slice);

    //文字列以外のスライス
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; //&[i32]という型になる
    for i in slice.iter() {
        print!("{} ", i);
    }
    println!("");
}

//与えられた文字列の最初の単語を返す
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
