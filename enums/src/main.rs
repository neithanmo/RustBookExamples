
enum Message {
    Quit,
    Move {x:i32, y:i32, z:u16 },
    Write(String),
    ChangeColor(u8,u8,u8),
}

fn check_value( x: Message){
    let result = match x {
        Message::Quit => println!("salgamos "),
        Message::Move{z:c, ..} => println!("Color struct third element is: {}", c),
        Message::Write(x) => println!("string data {}", x),
        Message::ChangeColor(_, _, y) => println!("changeColor {}", y),
    };
    println!("{:?}", result);

}

fn main() {
    println!("Hello, world!");
    let m = Message::Move{x:250, y:325, z:25};
    let m2 = Message::ChangeColor(214, 32, 49);
    check_value(m);
    check_value(m2);
}
