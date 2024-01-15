mod types;
mod instance;
#[cfg(feature = "util")]
pub mod util;
pub mod target;
pub mod geometry;
pub use types::*;
pub use instance::*;


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
