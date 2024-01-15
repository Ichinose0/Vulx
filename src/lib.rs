mod device;
pub mod geometry;
mod image;
mod instance;
mod queue;
pub mod target;
mod types;
#[cfg(feature = "util")]
pub mod util;
pub use device::*;
pub use image::*;
pub use instance::*;
pub use queue::*;
pub use types::*;

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
