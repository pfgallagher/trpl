fn main() {
    // let s = "hello";

    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}", s);

    // let x = 5;
    // let y = x; // x's value is copied into y and also stored on the stack.

    // println!("x = {}, y = {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1; // results in a move
    // let s2 = s1.clone(); // Creates a copy of the corresponding heap data.

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s); // s was moved into takes_ownership, so we can't use it now
    // let x = 5;
    // makes_copy(x);
    // println!("{}", x); // x is a copy, so we can still use it.

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);

    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}", s1, len);

    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);

    // let reference_to_nothing = dangle();
    // let s1 = no_dangle();
    // println!("{}", s1);

    // let s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..11];

    // let s = String::from("hello");
    // let slice1 = &s[0..2];
    // let slice2 = &s[..2]; // equivalent to slice1

    // let s = String::from("hello");
    // let len = s.len();
    // let slice1 = &s[3..len];
    // let slice2 = &s[3..]; // equivalent to slice1

    // let s = String::from("hello");
    // let len = s.len();
    // let slice1 = &s[0..len];
    // let slice2 = &s[..]; // equivalent to slice1

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // Throws error since we're trying to borrow a mutable reference when an immutable one already exists.
    // println!("the first word is: {}", word);

    // let my_string = String::from("hello world");
    // let word = first_word(&my_string[..]);
    // let my_string_literal = "hello world";
    // let word2 = first_word(&my_string_literal[..]);
    // let word3 = first_word(my_string_literal);

    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];
}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // Does not work as some_string is not mutable
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
