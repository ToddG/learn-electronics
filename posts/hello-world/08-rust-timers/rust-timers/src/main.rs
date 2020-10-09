#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_uno::hal::port;
use arduino_uno::prelude::*;
use core::mem;
use panic_halt as _;

static mut LED: mem::MaybeUninit<port::portb::PB5<port::mode::Output>> = mem::MaybeUninit::uninit();

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let led = pins.d13.into_output(&mut pins.ddr);
    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        LED = mem::MaybeUninit::new(led);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

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
    timer1.tccr1a.write(|w| unsafe { w.bits(0) });
    timer1.tccr1b.write(|w| w.cs1().prescale_256());
    timer1.ocr1a.write(|w| unsafe { w.bits(62499) });
    timer1.tcnt1.write(|w| unsafe {w.bits(0)});

    // Enable the timer interrupt
    timer1.timsk1.write(|w| w.ocie1a().set_bit());

    // Enable interrupts globally
    unsafe {
        // SAFETY: Not inside a critical section and any non-atomic operations have been completed
        // at this point.
        avr_device::interrupt::enable();
    }

    loop {
        avr_device::asm::sleep();
    }
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    let led = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
        // initialized so this ISR will never run when LED is uninitialized.
        &mut *LED.as_mut_ptr()
    };

    led.toggle().void_unwrap();
}
