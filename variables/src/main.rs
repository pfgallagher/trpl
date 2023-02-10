fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let five_hundred = tup.0;

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    another_function();
}

fn another_function() {
    println!("Another function.");
}
