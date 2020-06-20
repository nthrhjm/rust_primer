use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //panic!("crash and burn");//panic!マクロでパニックを起こす
    let v = vec![1, 2, 3];
    //v[99]; //ここでパニックを起こす
    //thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99',
    // /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libcore/slice/mod.rs:2791:10

    //Resultで回復可能なエラー
    //Result enumはOkとErrの2列挙子からなるように定義されている
    // enum Result<T, E>{
    //     Ok(T),
    //     Err(E),
    // }

    //ファイルを開こうとするコード
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a probrem opening the file: {:?}", error),
    };
}
