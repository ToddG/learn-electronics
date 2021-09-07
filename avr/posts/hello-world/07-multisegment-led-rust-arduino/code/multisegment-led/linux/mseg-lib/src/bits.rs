///! TODO: 1. introduce generics so that the input array
///! TODO: can be any size. currently hardcoded to usize.
///!
///! TODO: 2. should I put in bounds checking? Right now the index could
///! TODO: rollover if we leftshift more than the 8 bits in usize...
///!
///! Notes/Links:
///! * https://immunant.com/blog/2020/01/bitfields/
///! * https://doc.rust-lang.org/reference/type-layout.html
///! * https://stackoverflow.com/questions/36061560/can-i-take-a-byte-array-and-deserialize-it-into-a-struct

/// Set a single bit in a bit array.
///
/// Examples:
///
/// ```
/// use mseg_lib::bits::set;
///
/// let b1 = set(0b00000000, 0, true);
/// assert_eq!(0b00000001, b1);
///
/// let b2 = set(b1, 7, true);
/// assert_eq!(0b10000001, b2);
///
/// let b3 = set(b2, 7, true);
/// assert_eq!(0b10000001, b3);
///
/// let b4 = set(b2, 7, false);
/// assert_eq!(0b00000001, b4);
/// ```
pub fn set(input: usize, index: usize, value: bool) -> usize {
    let mask: usize = 1 << index;
    return if value {
        input | mask
    } else {
        input & !(mask)
    };
}

/// Retrieve a single bit from a bit array.
///
/// Examples:
///
/// ```
/// use mseg_lib::bits::get;
///
/// let b1 = get(0b00000001, 0);
/// assert_eq!(b1, true);
///
/// let b2 = get(0b00000001, 1);
/// assert_eq!(b2, false);
/// ```
pub fn get(input: usize, index: usize) -> bool {
    let mask: usize = 1 << index;
    let mut result = input & mask;
    result >>= index;
    return if result == 1 { true } else { false };
}

#[cfg(test)]
mod tests {
    #[test]
    /// set each bit on, one at a time, and verify that all the other bits are off.
    fn test_bits_set() {
        let b: usize = 0b00000000;
        use super::*;

        for i in 0..8 {
            for j in 0..8 {
                let actual = get(set(b, i, true), j);
                if i == j {
                    assert_eq!(true, actual);
                } else {
                    assert_eq!(false, actual);
                }
            }
        }
    }
}
