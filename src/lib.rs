pub mod scale;
pub mod note;
pub mod svg_draw;
pub mod tuning;
mod interval;

extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate serde;
extern crate serde_derive;

/// The number of degree is the position from tonic in scale
type Degree = u8;
pub const NUMBER_STRING: u8 = 6;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
