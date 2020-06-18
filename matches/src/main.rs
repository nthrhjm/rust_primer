#[derive(Debug)]
enum UsState {
    //アメリカの州名
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
//Option<i32>を取り値がなければNoneを返し、値があればその値に1を足して返す関数
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let alaska = Coin::Quarter(UsState::Alaska);
    value_in_cents(alaska);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is {:?}", six); //six is Some(6)
    println!("none is {:?}", none); //none is None
}
