#[derive(Debug)] //add functionality to print out information, this makes that functionality available for our struct
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area( &self ) -> u32 {
        self.width * self.height
    }
    fn can_hold( &self, other: &Rectangle ) -> bool { //rectangle method(Rectangle.can_hold())
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle { //associated function Rectangle::square()
        Rectangle { width: size, height: size }
    }
}

fn main() {


    let rectangle = Rectangle{
        width:30,
        height:150
    };
    println!(" rectangle is: {:#?}", rectangle);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(125384);
    println!(" rectangle sq: {:#?}", sq);
}
