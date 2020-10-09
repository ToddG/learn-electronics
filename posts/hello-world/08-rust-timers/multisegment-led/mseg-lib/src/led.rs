///! led module
///!
///! # Links:
///! * https://doc.rust-lang.org/book/ch10-02-traits.html
///! * https://doc.rust-lang.org/rust-by-example/trait.html
///! * https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
///! * https://doc.rust-lang.org/std/keyword.impl.html

/// SM4105W6 Eight Segment LED
    /// https://www.velleman.eu/downloads/29/infosheets/vmp502_sma42056etc.pdf
    ///
    ///  Hardware Pins
    ///      A  B   C   D   E   F   G   DP  COM
    ///      7  6   4   2   1   9   10  5   3/8
    ///
    /// In the array, the pins are indexed as:
    ///      A  B   C   D   E   F   G   DP  COM
    ///      0  1   2   3   4   5   6   7   8
#[derive(Clone, Copy, Debug)]
pub struct EightSegmentLEDCommonAnode {
    // pins (0-7 are A-DP, 8 is com)
    pub pins: [usize; 9],
}

impl EightSegmentLEDCommonAnode {
    pub fn pins(&self) -> [usize;9] {
        self.pins
    }
    pub fn com(&self) -> usize {
        self.pins[8]
    }
}
