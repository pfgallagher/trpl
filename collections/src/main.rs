use std::collections::HashMap;

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }
fn main() {
    // let v: Vec<i32> = Vec::new();

    // let mut v = vec![1, 2, 3];

    // let mut v = Vec::new();
    // v.push(4);
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1,2,3,4,5];
    // let third = &v[2];
    // let third2 = v.get(2);

    // let v = vec![1,2,3,4,5];
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // let mut v = vec![1,2,3,4,5];
    // let first = &v[0];

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // let mut s = String::new();

    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    // let mut s = String::from("lo");
    // s.push('l');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{}-{}-{}", s1, s2, s3);

    // let s1 = String::from("hello");
    // let h = s1[0];

    // let len = String::from("Hola").len();

    // You specify bytes in string slice ranges, not scalar values or graphemes.

    // For iteration, you can use string `.chars` or `.bytes` - graphemes require an external crate

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{}, {}", field_name, field_value);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{:?}", scores);

    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:?}", map);
}
