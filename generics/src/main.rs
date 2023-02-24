struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
fn main() {
    // let number_list = vec![32, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let invalid = Point { x: 5, y: 4.0 };
    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };
    // let p3 = p1.mixup(p2);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let integer = Some(5);
    // let float = Some(5.0);
}

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if largest < item {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if largest < item {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if largest < item {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if largest < item {
//             largest = item;
//         }
//     }
//     largest
// }
