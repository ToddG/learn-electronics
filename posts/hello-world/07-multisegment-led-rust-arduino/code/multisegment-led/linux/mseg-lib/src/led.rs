use crate::bits;
use crate::cmap;
///! led module
///!
///! # Links:
///! * https://doc.rust-lang.org/book/ch10-02-traits.html
///! * https://doc.rust-lang.org/rust-by-example/trait.html
///! * https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
///! * https://doc.rust-lang.org/std/keyword.impl.html

use crate::hal;

/// SM4105W6 Eight Segment LED
    /// https://www.velleman.eu/downloads/29/infosheets/vmp502_sma42056etc.pdf
    ///
    ///  Pins
    ///      A    B   C   D   E   F   G   DP
    ///      7    6   4   2   1   9   10  5
    ///
pub struct EightSegmentLEDCommonAnode {
    // com is either 3 or 8
    pcom: usize,
    // digital IO pins
    p7a: usize,
    p6b: usize,
    p4c: usize,
    p2d: usize,
    p1e: usize,
    p9f: usize,
    p10g: usize,
    p5dp: usize,
    // data to display
    data: usize,
    // hardware abstraction layer
    hal: Box<dyn hal::HAL>,
}

/// Construct new EightSegmentLEDCommonAnode
///
/// # Example
///
/// ```
/// use mseg_lib::led::new_eight_segment_led_common_anode;
/// use mseg_lib::platform::*;
/// use mseg_lib::hal::*;
///
/// //For testing, use the x86 platform
/// let my_hal = x86::new_hal(10);
///
/// let led = new_eight_segment_led_common_anode(1,2,3,4,5,6,7,8,9,my_hal);
/// ```
pub fn new_eight_segment_led_common_anode(
    led_common_anode_digital_iopin: usize,
    led_pin01e_digital_iopin: usize,
    led_pin02d_digital_iopin: usize,
    led_pin04c_digital_iopin: usize,
    led_pin05dp_digital_iopin: usize,
    led_pin06b_digital_iopin: usize,
    led_pin07a_digital_iopin: usize,
    led_pin09f_digital_iopin: usize,
    led_pin10g_digital_iopi: usize,
    hal: Box<dyn hal::HAL>,
) -> EightSegmentLEDCommonAnode {
    EightSegmentLEDCommonAnode {
        pcom: led_common_anode_digital_iopin,
        p7a: led_pin07a_digital_iopin,
        p6b: led_pin06b_digital_iopin,
        p4c: led_pin04c_digital_iopin,
        p2d: led_pin02d_digital_iopin,
        p1e: led_pin01e_digital_iopin,
        p9f: led_pin09f_digital_iopin,
        p10g: led_pin10g_digital_iopi,
        p5dp: led_pin05dp_digital_iopin,
        hal,
        data: cmap::CHAR_SPACE,
    }
}

impl EightSegmentLEDCommonAnode {
    pub fn dump(&self) -> Vec::<String> {
        let mut v = self.hal.dump();
        v.push(format!("led data: {}", self.data));
        v
    }

    pub fn set(&mut self, input: usize) {
        self.data = input;
    }

    pub fn init(&mut self) {
        // configure pins
        self.hal.pin_mode(self.pcom.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p1e.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p2d.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p4c.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p5dp.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p6b.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p7a.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p9f.into(), hal::PinMode::OUTPUT);
        self.hal.pin_mode(self.p10g.into(), hal::PinMode::OUTPUT);
        // turn all the led segments off
        self.hal.digital_write(self.pcom.into(), false);
        self.hal.digital_write(self.p1e.into(), true);
        self.hal.digital_write(self.p2d.into(), true);
        self.hal.digital_write(self.p4c.into(), true);
        self.hal.digital_write(self.p5dp.into(), true);
        self.hal.digital_write(self.p6b.into(), true);
        self.hal.digital_write(self.p7a.into(), true);
        self.hal.digital_write(self.p9f.into(), true);
        self.hal.digital_write(self.p10g.into(), true);
    }

    pub fn strobe(&mut self) {
        let led = cmap::segments(self.data.into());
        self.hal.digital_write(self.pcom.into(), true);
        self.hal.digital_write(self.p7a.into(), !bits::get(led, cmap::SEGMENT_INDEX_A));
        self.hal.digital_write(self.p6b.into(), !bits::get(led, cmap::SEGMENT_INDEX_B));
        self.hal.digital_write(self.p4c.into(), !bits::get(led, cmap::SEGMENT_INDEX_C));
        self.hal.digital_write(self.p2d.into(), !bits::get(led, cmap::SEGMENT_INDEX_D));
        self.hal.digital_write(self.p1e.into(), !bits::get(led, cmap::SEGMENT_INDEX_E));
        self.hal.digital_write(self.p9f.into(), !bits::get(led, cmap::SEGMENT_INDEX_F));
        self.hal.digital_write(self.p10g.into(), !bits::get(led, cmap::SEGMENT_INDEX_G));
        self.hal.digital_write(self.p5dp.into(), !bits::get(led, cmap::SEGMENT_INDEX_DP));
        self.hal.digital_write(self.pcom.into(), false);
    }
}
