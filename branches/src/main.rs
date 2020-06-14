fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!\n");
    //上のwhile文と同じ
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!\n");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("");
    //上のwhile文と同じ
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    println!("");
}
