#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // These methods could be in separate impl blocks all named Rectangle, if desired.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
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
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // let rect4 = Rectangle::square(3);

    // println!("{:#?}", rect1);
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
