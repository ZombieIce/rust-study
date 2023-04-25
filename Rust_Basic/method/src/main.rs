// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl Circle {
//     fn new(x: f64, y: f64, radius: f64) -> Circle {
//         Circle { x: x, y: y, radius: radius }
//     }

//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 60, height: 45};
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
