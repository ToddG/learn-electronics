///! `cmap` module handles character maps.
///!
///!
///! Generic 8 Segment LED
///!
///!  +--------------------------------------------------------------+
///! |                                                              |
///! |                                                              |
///! |                        A                                     |
///! |            |----------------------------|                    |
///! |            |----------------------------|                    |
///! |            +--+                      +--+                    |
///! |            |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |            |  |                      |  | B                  |
///! |      F     |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |            +--+          G           +--+                    |
///! |            |----------------------------|                    |
///! |            |----------------------------|                    |
///! |            +--+                      +--+                    |
///! |            |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |       E    |  |                      |  | C                  |
///! |            |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |            |  |                      |  |                    |
///! |            +--+                      +--+                    |
///! |            |----------------------------|      +-+           |
///! |            |----------------------------|      +-+   DP      |
///! |                        D                                     |
///! |                                                              |
///! +--------------------------------------------------------------+
///!

// segment indexes
pub const SEGMENT_INDEX_A: usize = 0;
pub const SEGMENT_INDEX_B: usize = 1;
pub const SEGMENT_INDEX_C: usize = 2;
pub const SEGMENT_INDEX_D: usize = 3;
pub const SEGMENT_INDEX_E: usize = 4;
pub const SEGMENT_INDEX_F: usize = 5;
pub const SEGMENT_INDEX_G: usize = 6;
pub const SEGMENT_INDEX_DP: usize = 7;

// numbers
pub const NUM_ZERO: usize = 0;
pub const NUM_ONE: usize = 1;
pub const NUM_TWO: usize = 2;
pub const NUM_THREE: usize = 3;
pub const NUM_FOUR: usize = 4;
pub const NUM_FIVE: usize = 5;
pub const NUM_SIX: usize = 6;
pub const NUM_SEVEN: usize = 7;
pub const NUM_EIGHT: usize = 8;
pub const NUM_NINE: usize = 9;

// characters
pub const CHAR_A: usize = 10;
pub const CHAR_B: usize = 11;
pub const CHAR_C: usize = 12;
pub const CHAR_D: usize = 13;
pub const CHAR_E: usize = 14;
pub const CHAR_F: usize = 15;
pub const CHAR_G: usize = 16;
pub const CHAR_H: usize = 17;
pub const CHAR_I: usize = 18;
pub const CHAR_J: usize = 19;
pub const CHAR_K: usize = 20;
pub const CHAR_L: usize = 21;
pub const CHAR_M: usize = 22;
pub const CHAR_N: usize = 23;
pub const CHAR_O: usize = 24;
pub const CHAR_P: usize = 25;
pub const CHAR_Q: usize = 26;
pub const CHAR_R: usize = 27;
pub const CHAR_S: usize = 28;
pub const CHAR_T: usize = 29;
pub const CHAR_U: usize = 30;
pub const CHAR_V: usize = 31;
pub const CHAR_W: usize = 32;
pub const CHAR_X: usize = 33;
pub const CHAR_Y: usize = 34;
pub const CHAR_Z: usize = 35;
pub const CHAR_SPACE: usize = 36;

// error
pub const ERROR: usize = 37;

pub const NUM_DISPLAY_ELEMENTS: usize = 38;

/// led segments (see SEGMENT_INDEX_XX)
/// 0bABCDEFG(DP)
///
/// TODO: This is probably not nearly as compact as I was hoping.
/// TODO: Examine generated binary and see if this is a packed `usize` or
/// TODO: if this is expanded into an array of `usize` elements. Documentation
/// TODO: suggests the latter, and that is why I have to call `.into()` to convert
/// TODO: from `usize` to `usize` when indexing.
///
/// LINKS:
/// * https://blog.yossarian.net/2020/05/20/Things-I-hate-about-rust
/// * https://doc.rust-lang.org/book/ch03-02-data-types.html
/// * https://doc.rust-lang.org/reference/items/type-aliases.html
/// * https://doc.rust-lang.org/reference/type-layout.html
/// * https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
/// * https://doc.rust-lang.org/std/primitive.array.html
/// * https://doc.rust-lang.org/std/primitive.bool.html#impl-BitAnd%3C%26%27_%20bool%3E
/// * https://doc.rust-lang.org/std/primitive.usize.html
/// * https://docs.rs/bit_field/0.10.0/bit_field/trait.BitArray.html
/// * https://docs.rs/bitvec/0.19.1/bitvec/#examples
/// * https://docs.rs/packed_struct/0.3.0/packed_struct/
/// * https://stackoverflow.com/questions/40259802/why-are-all-indexes-in-rust-of-type-usize
///
/// Interesting discussion re: size of binaries. Useful as I'm examining the
/// binary size of `mseg-lib` to see how DISPLAY_SEGMENTS is getting laid out in the
/// binary.
/// * https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html
const DISPLAY_SEGMENTS:[usize; NUM_DISPLAY_ELEMENTS] = [
    0b11111100, // NUM_ZERO
    0b01100000, // NUM_ONE
    0b11011010, // NUM_TWO
    0b11110010, // NUM_THREE
    0b01100110, // NUM_FOUR
    0b10110110, // NUM_FIVE
    0b10111110, // NUM_SIX
    0b11100000, // NUM_SEVEN
    0b11111110, // NUM_EIGHT
    0b11100110, // NUM_NINE
    0b11101111, // CHAR_A
    0b11111111, // CHAR_B
    0b10011101, // CHAR_C
    0b11111101, // CHAR_D
    0b10011111, // CHAR_E
    0b10001111, // CHAR_F
    0b10111111, // CHAR_G
    0b01101111, // CHAR_H
    0b11110001, // CHAR_I
    0b01111001, // CHAR_J
    0b00101101, // CHAR_K
    0b00011101, // CHAR_L
    0b11101101, // CHAR_M
    0b00101011, // CHAR_N
    0b11111101, // CHAR_O
    0b11001111, // CHAR_P
    0b10010011, // CHAR_Q
    0b11101111, // CHAR_R
    0b10110111, // CHAR_S
    0b11110001, // CHAR_T
    0b01111101, // CHAR_U
    0b00111001, // CHAR_V
    0b10010011, // CHAR_W
    0b01101101, // CHAR_X
    0b01001111, // CHAR_Y
    0b11011011, // CHAR_Z
    0b00000000, // CHAR_SPACE
    0b11111111, // ERROR
];

/// Return the segments for a given character (CHAR_*) or a given
/// number (NUM_*). Segments is the smallest data type that can
/// encode an 8 segment LED display, so a `usize`.
///
/// Use the bits::set and bits::get with the SEGMENT_INDEX_* constants
/// defined above.
///
/// # Example
///
/// ```
/// use mseg_lib::cmap::*;
/// use mseg_lib::bits;
/// let x = segments(NUM_ZERO.into());
/// // NUM_ZERO is 0b11111100, so let's check it
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_A), bits::get(x, SEGMENT_INDEX_A));
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_B), bits::get(x, SEGMENT_INDEX_B));
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_C), bits::get(x, SEGMENT_INDEX_C));
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_D), bits::get(x, SEGMENT_INDEX_D));
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_E), bits::get(x, SEGMENT_INDEX_E));
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_F), bits::get(x, SEGMENT_INDEX_F));
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_G), bits::get(x, SEGMENT_INDEX_G));
/// assert_eq!(bits::get(0b11111100, SEGMENT_INDEX_DP), bits::get(x, SEGMENT_INDEX_DP));
/// ```
pub fn segments(input: usize) -> usize {
    return if input < NUM_DISPLAY_ELEMENTS - 1 {
        DISPLAY_SEGMENTS[input]
    } else {
        DISPLAY_SEGMENTS[ERROR]
    }
}
