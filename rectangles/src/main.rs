//構造体を使ったプログラム例
#[derive(Debug)] //Rectangle構造体をDebugトレイトを継承（Rectangleインスタンスをデバッグ用整形機で出力するため）
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        //メソッドの定義
        self.width * self.height
    }
    //もう一つのRectangle構造体の不変参照を受け取り
    //自身のRectanngle構造体の中に収まるかを返す関数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //関連関数
    //implブロック内に定義されるがselfをパラメーターとして取らない関数（メソッドではない）
    //関連関数は構造体の新規インスタンスを返すコンストラクタによく使用される。
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The are of the rectangle is {} square pixels.\n",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.\n",
        tpl_area(rect1)
    );

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };
    //println!マクロで{:?}や{:#?}を使うとデバック出力用に整形される
    println!("rect2 is {:?}", rect2); //rect2 is Rectangle { width: 40, height: 50 }
    println!("rect2 is {:#?}", rect2); // rect2 is Rectangle {
                                       // width: 40,
                                       // height: 50,
                                       //}
    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect2)
    );

    let rect3 = Rectangle {
        width: 50,
        height: 90,
    };

    //Rectangle構造体に定義されたarea()methodを使用する
    println!("The area of the rect3 is {} square pixels.", rect3.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect3.can_hold(&rect3));

    //関連関数square()を呼び出す
    //関連関数の呼び出しは`構造体::関連関数`という記法を使う
    println!("sq is {:?}", Rectangle::square(3)); //sq is Rectangle { width: 3, height: 3 }
}
//パラメーターをふたつ取る関数
fn area(width: u32, height: u32) -> u32 {
    width * height
}
//タプルでパラメーターを受け取る関数
fn tpl_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
//パラメーターを構造体で受け取る関数
fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
