#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_uno::hal::port;
use arduino_uno::prelude::*;
use arr_macro::arr;
use core::cell::UnsafeCell;
use core::mem;
use panic_halt as _;
use avr_device::interrupt;
use mseg_lib::{cmap, bits};

/// ------------------------------------------------------------------------
/// Consts
/// Modify these values based on the number of LEDs:
/// ------------------------------------------------------------------------
/// MAX_VALUE : if 1 LED, then this would be 10, 2 leds then 100, 3 leds then 1000.
/// This is obviously pow(10, NUM_LEDS).
const MAX_VALUE: usize = 100;

/// The number of LEDS in the display. Common numbers are 1, 2, 6, etc. Whatever
/// you have wired up.
const NUM_LEDS: usize = 2;

/// Always gonna be 10
const BASE: usize = 10usize;

/// This is where you wire up the LED segments to the GPIO pins.
const LEDS_COMMON_CATHODES: [usize; 8] = [2, 3, 4, 5, 6, 7, 8, 9];
const LED_ANODES: [usize; 2] = [10, 11];

/// ------------------------------------------------------------------------
/// State
/// ------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
struct State {
    _led_to_strobe_index: usize,
    _display_counter: usize,
}

impl State {
    pub fn increment_led_to_strobe(&self) -> usize {
        (self._led_to_strobe_index + 1) % NUM_LEDS
    }
    pub fn increment_counter(&self) -> usize {
        (self._display_counter + 1) % MAX_VALUE
    }
    pub fn led_to_strobe(&self) -> usize {
        self._led_to_strobe_index
    }
    pub fn counter(&self) -> usize {
        self._display_counter
    }
    pub fn digits(&self) -> [u8; NUM_LEDS] {
        let mut num = self._display_counter;
        let mut result = [0; NUM_LEDS];
        // extract each digit from the count up counter and update the display value accordingly
        for i in 0..NUM_LEDS {
            result[i] = (num % BASE) as u8;
            num /= BASE
        }
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
static mut PINS: mem::MaybeUninit<[port::Pin<port::mode::Output>; 10]> = mem::MaybeUninit::uninit();
static GLOBAL_STATE: GlobalState = GLOBAL_STATE_INIT;


#[arduino_uno::entry]
fn main() -> ! {
    configure_hardware();

    unsafe {
        // !!!COMPILER FENCE!!!
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        // Enable interrupts globally
        avr_device::interrupt::enable();
    }

    loop {
        avr_device::asm::sleep();
    }
}

fn configure_hardware() {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );


    let led = pins.d13.into_output(&mut pins.ddr).downgrade();
    // let mut pd0 = pins.d0.into_output(&mut pins.ddr); // rx
    // let mut pd1 = pins.d1.into_output(&mut pins.ddr); // tx
    let pd02 = pins.d2.into_output(&mut pins.ddr).downgrade();
    let pd03 = pins.d3.into_output(&mut pins.ddr).downgrade();
    let pd04 = pins.d4.into_output(&mut pins.ddr).downgrade();
    let pd05 = pins.d5.into_output(&mut pins.ddr).downgrade();
    let pd06 = pins.d6.into_output(&mut pins.ddr).downgrade();
    let pd07 = pins.d7.into_output(&mut pins.ddr).downgrade();
    let pd08 = pins.d8.into_output(&mut pins.ddr).downgrade();
    let pd09 = pins.d9.into_output(&mut pins.ddr).downgrade();
    let pd10 = pins.d10.into_output(&mut pins.ddr).downgrade();
    let pd11 = pins.d11.into_output(&mut pins.ddr).downgrade();
    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global (static)
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        LED = mem::MaybeUninit::new(led);
        PINS = mem::MaybeUninit::new([pd02, pd03, pd04, pd05, pd06, pd07, pd08, pd09, pd10, pd11]);
        // !!!COMPILER FENCE!!!
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    // // TIMER0 : 1kHz
    // // # ------------------------------------
    // // # interrupt frequency: f
    // // # prescaler: p
    // // # compare match register value: cmr
    // // # timers: t
    // // # ------------------------------------
    // // "f: 1000, p: 64, cmr: 249.0, t: [0, 2]"
    let timer0 = peripherals.TC0;
    timer0.tccr0a.write(|w| unsafe { w.bits(0) });
    timer0.tccr0b.write(|w| w.cs0().prescale_64());
    timer0.tcnt0.write(|w| unsafe { w.bits(0) });
    // Enable the timer interrupt
    timer0.timsk0.write(|w| w.ocie0a().set_bit());

    // TIMER1 : 1Hz
    // # ------------------------------------
    // # interrupt frequency: f
    // # prescaler: p
    // # compare match register value: cmr
    // # timers: t
    // # ------------------------------------
    // "f: 1, p: 256, cmr: 62499.0, t: [1]"
    let timer1 = peripherals.TC1;
    timer1.tccr1a.write(|w| unsafe { w.bits(0) });
    timer1.tccr1b.write(|w| w.cs1().prescale_256());
    timer1.ocr1a.write(|w| unsafe { w.bits(62499) });
    timer1.tcnt1.write(|w| unsafe { w.bits(0) });
    // Enable the timer interrupt
    timer1.timsk1.write(|w| w.ocie1a().set_bit());
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    let pins = unsafe {
        &mut *PINS.as_mut_ptr()
    };
    let state = interrupt::free(|cs| GLOBAL_STATE.get(cs));
    let strobe_idx = state.led_to_strobe();

    // set COM HIGH for the led to strobe
    let target_led_com = LED_ANODES[strobe_idx];
    for com in LED_ANODES.iter() {
        let xcom = *com - 2;
        if xcom == target_led_com {
            if pins[xcom].is_set_low().void_unwrap() {
                pins[xcom].set_high().void_unwrap();
            }
        } else {
            if pins[xcom].is_set_high().void_unwrap() {
                pins[xcom].set_low().void_unwrap();
            }
        }
    }

    // set the segments to high and low for this digit
    let digit = state.digits()[strobe_idx];
    let segments = cmap::segments(digit);
    for x in 0..8{
        let segment = bits::get(segments, x);
        let cathode = LEDS_COMMON_CATHODES[x];
        let xcat = cathode - 2;
        if segment && pins[xcat].is_set_low().void_unwrap(){
            pins[xcat].set_high().void_unwrap();
        }else if !segment && pins[xcat].is_set_high().void_unwrap() {
            pins[xcat].set_low().void_unwrap();
        }
    }
    let next_state = State { _led_to_strobe_index: state.increment_led_to_strobe(), ..state };
    interrupt::free(|cs| GLOBAL_STATE.update(next_state, cs));
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    let state = interrupt::free(|cs| GLOBAL_STATE.get(cs));
    let next_state = State { _display_counter: state.increment_counter(), ..state };
    interrupt::free(|cs| GLOBAL_STATE.update(next_state, cs));
    let led = unsafe {
        &mut *LED.as_mut_ptr()
    };
    led.toggle().void_unwrap();
}