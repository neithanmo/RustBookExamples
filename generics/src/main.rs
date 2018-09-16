// generic data type
struct Point<T> {
    x: T,
    y: T,
}

//generic implementation of method x for generic struct Point
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
// which only can be f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//********************************************************
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
//*******************************************************



fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point{x: 25.25, y: 52.25};

    println!("p.x = {}", p.x());

    println!("distance {}", p2.distance_from_origin() );

    let p3 = Point2 {x: 10, y:"como estas" };
    print!("p3 {:#?}", p3);
    let p4 = Point2 { x:"hola", y: 52.25 };
    print!("p4 {:#?}", p4);
    let p5 = p4.mixup(p3);
    print!("p5 {:#?}", p5);
}
