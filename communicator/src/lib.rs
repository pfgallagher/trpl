pub mod client;

pub mod network;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
