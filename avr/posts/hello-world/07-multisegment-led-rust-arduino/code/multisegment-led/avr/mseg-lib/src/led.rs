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
pub struct EightSegmentLEDCommonAnode {
    // pins (0-7 are A-DP, 8 is com)
    _pins: [usize; 9],
    // data to display
    _data: u8,
}

pub fn new_eight_segment_led_common_anode(pins: [usize; 9]) -> EightSegmentLEDCommonAnode {
    EightSegmentLEDCommonAnode {
        _pins: pins,
        _data: 0,
    }
}

impl EightSegmentLEDCommonAnode {
    pub fn update(&mut self, data: u8) { self._data = data; }
    pub fn data(&self) -> u8 { self._data }
    pub fn pins(&self) -> [usize; 9] { self._pins }
    pub fn com(&self) -> usize { self._pins[8]}
}
