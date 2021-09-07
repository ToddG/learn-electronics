#![no_std]
#![no_main]
extern crate mseg_lib;
extern crate panic_halt;

use arduino_uno::hal::port::mode;
use arduino_uno::hal::port::Pin;
use arduino_uno::prelude::*;

use mseg_lib::{bits, cmap};
use mseg_lib::led::*;

struct Hardware {
    pub onboard_led_pin: Pin<mode::Output>,
    pub pins: [Pin<mode::Output>; 10],
    pub leds: [EightSegmentLEDCommonAnode; 2],
    pub idx: usize,
    pub display: usize,
}

fn handle_timer0_1k_hz(mut hardware: Hardware) -> Hardware {
    // turn off all leds
    for x in 0..2 {
        let com_pin = hardware.leds[x].com();
        hardware.pins[com_pin - 2].set_low().void_unwrap();
    }
    // alternately turn on one of the leds
    hardware.idx = (hardware.idx + 1) % 2;

    // borrow the target led from the leds struct
    let target_led = &mut hardware.leds[hardware.idx];

    // get the currently displayed value and segments for this led
    let target_led_segments = cmap::segments(target_led.data());

    // gross code to iterate over the segments (and com) and set the pins
    let target_led_pins = target_led.pins();
    for segment_idx in 0..8 {
        let gpio_pin = target_led_pins[segment_idx];
        let display_segment = bits::get(target_led_segments, segment_idx);
        if display_segment {
            // cathode pins go low to display
            hardware.pins[gpio_pin - 2].set_low().void_unwrap()
        } else {
            // cathode pins go high to turn off
            hardware.pins[gpio_pin - 2].set_high().void_unwrap()
        }
    }
    // set this one led on
    hardware.pins[target_led.com() - 2].set_high().void_unwrap();
    hardware
}


// moves hardware in, does some magic, and move hardware back out
fn handle_timer1_1hz(mut hardware: Hardware) -> Hardware {
    hardware.onboard_led_pin.toggle().void_unwrap();
    // alternately turn on one of the leds
    hardware.display = (hardware.display + 1) % 100;
    let v1 = (hardware.display / 10 ) % 10;
    let v2 = (hardware.display - (v1 * 10)) % 10;
    let values = [v1, v2];
    for led_idx in 0..2 {
        let target_led = &mut hardware.leds[led_idx];
        // update the target leds
        target_led.update(values[led_idx] as u8);
    }
    hardware
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let led = pins.d13.into_output(&mut pins.ddr);
    // let mut pd0 = pins.d0.into_output(&mut pins.ddr); // rx
    // let mut pd1 = pins.d1.into_output(&mut pins.ddr); // tx
    let pd2 = pins.d2.into_output(&mut pins.ddr);
    let pd3 = pins.d3.into_output(&mut pins.ddr);
    let pd4 = pins.d4.into_output(&mut pins.ddr);
    let pd5 = pins.d5.into_output(&mut pins.ddr);
    let pd6 = pins.d6.into_output(&mut pins.ddr);
    let pd7 = pins.d7.into_output(&mut pins.ddr);
    let pd8 = pins.d8.into_output(&mut pins.ddr);
    let pd9 = pins.d9.into_output(&mut pins.ddr);
    let pd10 = pins.d10.into_output(&mut pins.ddr);
    let pd11 = pins.d11.into_output(&mut pins.ddr);

    let led8a =
        // A B C D E F G DP COM
        new_eight_segment_led_common_anode(
            [2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let led8b =
        // A B C D E F G DP COM
        new_eight_segment_led_common_anode(
            [2, 3, 4, 5, 6, 7, 8, 9, 11]);

    let mut hardware = Hardware {
        leds: [led8a, led8b],
        onboard_led_pin: led.downgrade(),
        pins: [
            pd2.downgrade(),
            pd3.downgrade(),
            pd4.downgrade(),
            pd5.downgrade(),
            pd6.downgrade(),
            pd7.downgrade(),
            pd8.downgrade(),
            pd9.downgrade(),
            pd10.downgrade(),
            pd11.downgrade(),
        ],
        idx: 0,
        display: 0
    };

    // configure timers

    // TIMER0 : 1kHz
    // # ------------------------------------
    // # interrupt frequency: f
    // # prescaler: p
    // # compare match register value: cmr
    // # timers: t
    // # ------------------------------------
    //     "f: 1000, p: 1, cmr: 15999.0, t: [1]"
    // "f: 1000, p: 8, cmr: 1999.0, t: [1]"
    // "f: 1000, p: 64, cmr: 249.0, t: [0, 2]"
    // "f: 1000, p: 256, cmr: 61.5, t: [0, 2]"
    // "f: 1000, p: 1024, cmr: 14.625, t: [0, 2]"
    let timer0 = peripherals.TC0;

    // #[doc = "0x0f - Timer/Counter Control Register A"]
    timer0.tccr0a.write(|w| unsafe { w.bits(0) });

    // prescale
    // #[doc = "0x10 - Timer/Counter Control Register B"]
    timer0.tccr0b.write(|w| w.cs0().prescale_64());

    // init timer0 count with zero
    // #[doc = "0x4e - Timer/Counter1 Bytes"]
    timer0.tcnt0.write(|w| unsafe { w.bits(0) });

    // TIMER1 : 1Hz
    // # ------------------------------------
    // # interrupt frequency: f
    // # prescaler: p
    // # compare match register value: cmr
    // # timers: t
    // # ------------------------------------
    // "f: 1, p: 1, cmr: 15999999.0, t: None"
    // "f: 1, p: 8, cmr: 1999999.0, t: None"
    // "f: 1, p: 64, cmr: 249999.0, t: None"
    // "f: 1, p: 256, cmr: 62499.0, t: [1]"
    // "f: 1, p: 1024, cmr: 15624.0, t: [1]"

    let timer1 = peripherals.TC1;
    // match
    // #[doc = "0x4a - Timer/Counter1 Control Register A"]
    timer1.tccr1a.write(|w| unsafe { w.bits(0) });
    // prescale
    // #[doc = "0x4b - Timer/Counter1 Control Register B"]
    timer1.tccr1b.write(|w| w.cs1().prescale_256());
    // init timer1 counter to zero
    // #[doc = "0x4e - Timer/Counter1 Bytes"]
    // timer1.tcnt1.write(|w| unsafe {w.bits(0)});
    // compare and match register
    //#[doc = "0x52 - Timer/Counter1 Output Compare Register Bytes"]
    // timer1.ocr1a.write(|w| unsafe {w.bits(62499)});

    // init timer1 with zero
    // #[doc = "0x4e - Timer/Counter1 Bytes"]
    timer1.tcnt1.write(|w| unsafe { w.bits(0) });

    // enter loop
    loop {
        // manual compare timer1 : 1Hz
        if timer1.tcnt1.read().bits() >= 62499 {
            // --------------------------------------------------
            // DO SOMETHING AS IF THIS WERE AN ISR FOR TIMER1
            // --------------------------------------------------
            hardware = handle_timer1_1hz(hardware);
            // reset timer
            timer1.tcnt1.write(|w| unsafe { w.bits(0) });
        }
        // manual compare timer0 : 1kHz
        if timer0.tcnt0.read().bits() >= 249 {
            // --------------------------------------------------
            // DO SOMETHING AS IF THIS WERE AN ISR FOR TIMER0
            // --------------------------------------------------
            hardware = handle_timer0_1k_hz(hardware);
            // reset timer
            timer0.tcnt0.write(|w| unsafe { w.bits(0) });
        }
    }
}

