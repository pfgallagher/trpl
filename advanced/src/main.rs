// use std::slice;

// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// static HELLO_WORLD: &str = "Hello, world!";
// static mut COUNTER: u32 = 0;

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// unsafe trait Foo {}

// unsafe impl Foo for i32 {}

// struct Context<'s>(&'s str);

// struct Parser<'c, 's: 'c> {
//     context: &'c Context<'s>,
// }
// impl<'c, 's> Parser<'c, 's> {
//     fn parse(&self) -> Result<(), &'s str> {
//         Err(&self.context.0[1..])
//     }
// }

// fn parse_context(context: Context) -> Result<(), &str> {
//     Parser { context: &context }.parse()
// }

// struct Ref<'a, T: 'a>(&'a T);

// struct StaticRef<T: 'static>(&'static T);

// trait Red {}

// struct Ball<'a> {
//     diameter: &'a i32,
// }
// impl<'a> Red for Ball<'a> {}

fn main() {
    // let mut num = 5;
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    // unsafe {
    //     println!("r1 is {}", *r1);
    //     println!("r2 is {}", *r2);
    //     dangerous();
    // }

    // let mut v = vec![1, 2, 3, 4, 5, 6];
    // let r = &mut v[..];
    // let (a, b) = split_at_mut(r, 3);
    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    // let address = 0x012345usize;
    // let r = address as *mut i32;
    // let slice = unsafe {
    //     slice::from_raw_parts_mut(r, 1000);
    // };

    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    // println!("name is: {}", HELLO_WORLD);

    // add_to_count(3);
    // unsafe {
    //     println!("COUNTER: {}", COUNTER);
    // }

    // let num = 5;
    // let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}

// unsafe fn dangerous() {}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid], &mut slice[mid..])
// }

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     let ptr = slice.as_mut_ptr();
//     assert!(mid <= len);
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
//         )
//     }
// }
