//! # mseg-lib
//!
//! `mseg-lib` is the library crate for the multi-segment led display.
//!
//!

pub mod cmap;
pub mod bits;
pub mod led;
pub mod hal;
pub mod platform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
