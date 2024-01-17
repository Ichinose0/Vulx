#![feature(offset_of)]

mod device;
pub mod geometry;
mod image;
mod instance;
mod pipeline;
mod queue;
mod renderpass;
mod shader;
pub mod target;
mod types;
#[cfg(feature = "util")]
pub mod util;
pub use device::*;
pub use image::*;
pub use instance::*;
pub use pipeline::*;
pub use queue::*;
pub use renderpass::*;
pub use shader::*;
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
