pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested modules");
            }
        }
    }
}
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
use a::series::of;
// use TrafficLight::{Red, Yellow};
use TrafficLight::*;
fn main() {
    //フルパスで関数呼び出しを書くのは長ったらしい
    a::series::of::nested_modules();
    //useキーワードでofモジュールまでスコープに導入すると下記のように書ける
    of::nested_modules();
    //enum TrafficLightをuseキーワードでスコープに導入すると
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
