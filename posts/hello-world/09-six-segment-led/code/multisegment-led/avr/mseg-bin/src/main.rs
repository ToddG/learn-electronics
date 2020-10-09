#![no_std]
#![no_main]
extern crate mseg_lib;
extern crate panic_halt;

use arduino_uno::prelude::*;

use mseg_lib::{bits, cmap};
use core::cell::UnsafeCell;
use core::mem;
use arduino_uno::hal::port;
use avr_device::interrupt;

/// ------------------------------------------------------------------------
/// Consts
/// Modify these values based on the number of LEDs:
/// ------------------------------------------------------------------------
/// MAX_VALUE : if 1 LED, then this would be 10, 2 leds then 100, 3 leds then 1000.
/// This is obviously pow(10, NUM_LEDS).
const MAX_VALUE: u32 = 100000;

/// The number of LEDS in the display. Common numbers are 1, 2, 6, etc. Whatever
/// you have wired up.
const NUM_LEDS: usize = 6;

/// Always gonna be 10
const BASE: u32 = 10;

/// This is where you wire up the LED segments to the GPIO pins.
const NUM_SEGMENTS: usize = 8;

/// ------------------------------------------------------------------------
/// State
/// ------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
struct State {
    _led_to_strobe_index: usize,
    _display_counter: u32,
}

impl State {
    pub fn increment_led_to_strobe(&self) -> usize {
        (self._led_to_strobe_index + 1) % NUM_LEDS
    }
    pub fn increment_counter(&self) -> u32 {
        (self._display_counter + 1) % MAX_VALUE
    }
    pub fn led_to_strobe(&self) -> usize {
        self._led_to_strobe_index
    }
    pub fn digits(&self) -> [u8; NUM_LEDS] {
        let mut num = self._display_counter;
        let mut result = [0; NUM_LEDS];
        // extract each digit from the count up counter and update the display value accordingly
        for i in 0..NUM_LEDS {
            result[i] = (num % BASE) as u8;
            num /= BASE
        }
        result.reverse();
        result
    }
}

struct GlobalState(UnsafeCell<State>);

impl GlobalState {
    pub fn get(&self, _cs: &avr_device::interrupt::CriticalSection) -> State {
        unsafe {
            let state = *self.0.get();
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            state
        }
    }
    pub fn update(&self, new_state: State, _cs: &avr_device::interrupt::CriticalSection) {
        unsafe {
            *self.0.get() = new_state;
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        };
    }
}

const GLOBAL_STATE_INIT: GlobalState = GlobalState(
    UnsafeCell::new(
        State {
            _led_to_strobe_index: 0,
            _display_counter: 0,
        }));

unsafe impl Sync for GlobalState {}

/// ------------------------------------------------------------------------
/// Statics
/// ------------------------------------------------------------------------
static mut LED: mem::MaybeUninit<port::Pin<port::mode::Output>> = mem::MaybeUninit::uninit();
static mut PINS: mem::MaybeUninit<[port::Pin<port::mode::Output>; NUM_SEGMENTS + NUM_LEDS]> = mem::MaybeUninit::uninit();
static GLOBAL_STATE: GlobalState = GLOBAL_STATE_INIT;

fn handle_timer0_1k_hz() {
    let pins = unsafe {
        &mut *PINS.as_mut_ptr()
    };
    // turn off all leds
    for x in 0..NUM_SEGMENTS {
        pins[x].set_high().void_unwrap();
    }
    for x in NUM_SEGMENTS..(NUM_SEGMENTS + NUM_LEDS) {
        pins[x].set_low().void_unwrap();
    }

    // get state
    let state: State = interrupt::free(|cs| GLOBAL_STATE.get(cs));

    // get segments
    let target_led_idx = state.led_to_strobe();
    let target_led_value = state.digits()[target_led_idx];
    let target_led_segments = cmap::segments(target_led_value);

    // iterate over the segments (and com) and set the pins
    for segment_idx in 0..NUM_SEGMENTS {
        let display_segment = bits::get(target_led_segments, segment_idx);
        if display_segment {
            // cathode pins go low to display
            pins[segment_idx].set_low().void_unwrap()
        } else {
            // cathode pins go high to turn off
            pins[segment_idx].set_high().void_unwrap()
        }
    }
    // set this one led on
    pins[NUM_SEGMENTS + target_led_idx].set_high().void_unwrap();

    // update state
    interrupt::free(|cs|
        GLOBAL_STATE.update(
            State {
                _led_to_strobe_index: state.increment_led_to_strobe(),
                ..state
            },
            cs));
}


fn handle_timer1_1hz() {
    let state = interrupt::free(|cs| GLOBAL_STATE.get(cs));
    let next_state = State { _display_counter: state.increment_counter(), ..state };
    interrupt::free(|cs| GLOBAL_STATE.update(next_state, cs));
    let led = unsafe {
        &mut *LED.as_mut_ptr()
    };
    led.toggle().void_unwrap();
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let led = pins.d13.into_output(&mut pins.ddr).downgrade();
    // let mut pd0 = pins.d0.into_output(&mut pins.ddr); // rx
    // let mut pd1 = pins.d1.into_output(&mut pins.ddr); // tx

    // common cathode
    let pd2 = pins.d2.into_output(&mut pins.ddr).downgrade();
    let pd3 = pins.d3.into_output(&mut pins.ddr).downgrade();
    let pd4 = pins.d4.into_output(&mut pins.ddr).downgrade();
    let pd5 = pins.d5.into_output(&mut pins.ddr).downgrade();
    let pd6 = pins.d6.into_output(&mut pins.ddr).downgrade();
    let pd7 = pins.d7.into_output(&mut pins.ddr).downgrade();
    let pd8 = pins.d8.into_output(&mut pins.ddr).downgrade();
    let pd9 = pins.d9.into_output(&mut pins.ddr).downgrade();

    // com pins (anodes 1:led)
    let pc0 = pins.a0.into_output(&mut pins.ddr).downgrade();
    let pc1 = pins.a1.into_output(&mut pins.ddr).downgrade();
    let pc2 = pins.a2.into_output(&mut pins.ddr).downgrade();
    let pc3 = pins.a3.into_output(&mut pins.ddr).downgrade();
    let pc4 = pins.a4.into_output(&mut pins.ddr).downgrade();
    let pc5 = pins.a5.into_output(&mut pins.ddr).downgrade();

    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global (static)
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        LED = mem::MaybeUninit::new(led);
        PINS = mem::MaybeUninit::new([pd2, pd3, pd4, pd5, pd6, pd7, pd8, pd9, pc0, pc1, pc2, pc3, pc4, pc5]);
        // !!!COMPILER FENCE!!!
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

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

    // TIMER1 : 10Hz
    // # ------------------------------------
    // # interrupt frequency: f
    // # prescaler: p
    // # compare match register value: cmr
    // # timers: t
    // # ------------------------------------
    // "f: 10, p: 1, cmr: 1599999.0, t: None"
    // "f: 10, p: 8, cmr: 199999.0, t: None"
    // "f: 10, p: 64, cmr: 24999.0, t: [1]"
    // "f: 10, p: 256, cmr: 6249.0, t: [1]"
    // "f: 10, p: 1024, cmr: 1561.5, t: [1]"

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
        if timer1.tcnt1.read().bits() >= 6249 {
            // --------------------------------------------------
            // DO SOMETHING AS IF THIS WERE AN ISR FOR TIMER1
            // --------------------------------------------------
            handle_timer1_1hz();
            // reset timer
            timer1.tcnt1.write(|w| unsafe { w.bits(0) });
        }
        // manual compare timer0 : 1kHz
        if timer0.tcnt0.read().bits() >= 249 {
            // --------------------------------------------------
            // DO SOMETHING AS IF THIS WERE AN ISR FOR TIMER0
            // --------------------------------------------------
            handle_timer0_1k_hz();
            // reset timer
            timer0.tcnt0.write(|w| unsafe { w.bits(0) });
        }
    }
}

