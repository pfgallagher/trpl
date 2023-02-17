pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

mod outermost {
    pub fn middle_function() {}
    pub fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {}
        pub fn secret_function() {}
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green
}

// use TrafficLight::{Red, Yellow};
use TrafficLight::*;
use a::series::of;
use a::series::of::nested_modules;

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
    a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
