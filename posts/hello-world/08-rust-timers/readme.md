# 08 Rust Timers

## Rust + Timers

The previous post showed how to use two timers within a loop sort of like this:

    loop {
        if timer1 { do something }
        if timer2 { do something }
    }


Pros: Simple. [No concurrency](https://docs.rust-embedded.org/book/concurrency/)
Cons: Callbacks are cooler, and may allow the CPU to perform less work, sleep more, and consume less power.

The callback strategy used by the Arduino C code examples looks more like:

    loop {
        ...configure timer1
        ...configure timer2
    }

    ISR(timer1) { do something };
    ISR(timer2) { do something };

Pros: Code for each timer callback is isolated.
Cons: Need to pass global state around and guard/control access.

## Summary

I was not able to get timers and interrupts to work reliably. The debugging
experience was really terrible, and the device behaviour was flakey. Sometimes
things worked, sometimes not... without any real rhyme or reason. Rahix
suggested that I am doing too much work in my ISRs and suggested a much simpler
strategy [here](https://github.com/Rahix/avr-hal/issues/75#issuecomment-702675340):

    (not tested)
    use core::sync::atomic;

    static TMR_OVERFLOW: atomic::AtomicBool = atomic::AtomicBool::new(false);

    #[avr_device::interrupt(atmega328p)]
    fn TIMER1_COMPA() {
        TMR_OVERFLOW.store(true, atomic::Ordering::SeqCst);
    }

    #[arduino_uno::entry]
    fn main() -> ! {
        // ...

        loop {
            let tmr_overflow = avr_device::interrupt::free(|_cs| {
                // With interrupts disabled, check and reset the flag.  If interrupts
                // were enabled, the ISR could run in between the check and reset, leading to
                // us loosing a timer overflow.
                if TMR_OVERFLOW.load(atomic::Ordering::SeqCst) {
                    TMR_OVERFLOW.store(false, atomic::Ordering::SeqCst);
                    true
                } else {
                    false
                }
            });

            if tmr_overflow {
                // the code you currently have in the ISR goes here.
            }
        }
    }

However, IHMO there is no real advantage to this pattern, as now you are dealing
with interrupts but the main loop is still running all the time. So really, why
bother?

### Details

So let's dig in and implement the multisegment LED display thingy using timers and ISRs!

## Code

### Timers in Rust

Let's start with [@Rahix's code from this thread](https://github.com/Rahix/avr-hal/issues/75#issuecomment-702675340):

```rust
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_leonardo::hal::port;
use arduino_leonardo::prelude::*;
use core::mem;
use panic_halt as _;

static mut LED: mem::MaybeUninit<port::portc::PC7<port::mode::Output>> = mem::MaybeUninit::uninit();

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let led = pins.d13.into_output(&mut pins.ddr);
    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        LED = mem::MaybeUninit::new(led);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    // Timer Configuration:
    // - WGM = 4: CTC mode (Clear Timer on Compare Match)
    // - Prescaler 256
    // - OCR1A = 15624
    //
    // => F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
    //     (^ this formula I deduced from reading the datasheet)
    //
    // => The LED will toggle at 4 Hz, thus it blinks at 2 Hz
    let tmr1 = dp.TC1;
    tmr1.tccr1a.write(|w| w.wgm1().bits(0b00));
    tmr1.tccr1b.write(|w| w.cs1().prescale_256().wgm1().bits(0b01));
    tmr1.ocr1a.write(|w| unsafe { w.bits(15624) });

    // Enable the timer interrupt
    tmr1.timsk1.write(|w| w.ocie1a().set_bit());

    // In theory this should not be necessary ... But if you previously had
    // a sketch from Arduino loaded, the USB device will not have been reset.
    // Because of this we will be spammed with interrupts which will never
    // stop because they are never handled.
    //
    // (only for ATmega32U4)
    dp.USB_DEVICE.usbcon.reset();

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

#[avr_device::interrupt(atmega32u4)]
fn TIMER1_COMPA() {
    let led = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
        // initialized so this ISR will never run when LED is uninitialized.
        &mut *LED.as_mut_ptr()
    };

    led.toggle().void_unwrap();
}
```


### Modifications

There's a couple of things I'm going to modify here...

* make this work with an 'uno' instead of a 'leonardo'
* change the timer config slightly

Here's what works for me [rust-timers/src/main.rs](./rust-timers/src/main.rs):

```rust
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
```

Note also the necessary changes to the various cargo config files, blah blah:

```bash
├── avr-atmega328p.json
├── .cargo
│   └── config.toml
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── rust-toolchain
└── src
    └── main.rs
```

## Multisegment Display

Now let's try to update the code from [the previous project](../07-multisegment-led-rust-arduino/readme.md) to use timers and global variables instead of the single `loop{}` construct. See [the source here](./multisegment-led/mseg-bin/src/main.rs):


Here's what the inner loop looks like to start:

```rust
    // configure timers
    // ...
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
```

Here's an outline of where we want to go:

```rust
    // configure timers
    // ...
    // enter loop
    loop {
        // some sort of sleep thingy
    }
    // timer1 ISR
    // timer2 ISR
```

### Code

I'm following the examples
[here](https://docs.rust-embedded.org/book/concurrency/).  The strategy is to
move the state out of the `Hardware` struct and into more fine grained structs
that encapsulate UnsafeCells. 


#### Top of File

Enable the interrupts and the unsafe cell:

```rust
#![feature(abi_avr_interrupt)]
use core::cell::UnsafeCell;
```

#### LedToStrobe

The `LedToStrobe` struct maintains the index of the next LED to turn on (or
strobe). This was extracted from `Hardware.idx`. Moving `idx` into
`LedToStrobe` enables threaded access, isolates the unsafe code, and provides a
nice clean API for accessing and incrementing the value:

```rust
const NUM_LEDS: usize = 2;

struct LedToStrobe(UnsafeCell<usize>);
const LED_TO_STROBE_INIT: LedToStrobe = LedToStrobe(UnsafeCell::new(0));

impl LedToStrobe{
    pub fn next(&self, _cs: &avr_device::interrupt::CriticalSection) -> usize {
        let idx = unsafe{ 
            *self.0.get() = (*self.0.get() + 1) % NUM_LEDS;
            *self.0.get()
        };
        idx
    }
}

unsafe impl Sync for LedToStrobe {}
static LED_TO_STROBE: LedToStrobe = LED_TO_STROBE_INIT;

```

Now when we want to strobe the next led, we invoke it like this:

```rust
// TODO: replace dummy crit sec when this is moved into a timer
let cs = unsafe { avr_device::interrupt::CriticalSection::new() };
let target_led = &mut hardware.leds[LED_TO_STROBE.next(&cs)];
```

Build and deploy and verify this still works, and it does!

```bash
make clean build deploy
```

#### DisplayCounter

Move the value to display out of `Hardware.display` and into `DisplayCounter`:

```rust
/// ------------------------------------------------------------------------
/// DisplayCounter encapsulates the upcounter value to be displayed
/// ------------------------------------------------------------------------
struct DisplayCounter(UnsafeCell<usize>);
const DISPLAY_COUNTER_INIT: DisplayCounter = DisplayCounter(UnsafeCell::new(0));

impl DisplayCounter{
    pub fn increment(&self, _cs: &avr_device::interrupt::CriticalSection) {
        unsafe{ 
            *self.0.get() = (*self.0.get() + 1) % MAX_VALUE;
        };
    }

    pub fn value(&self, _cs: &avr_device::interrupt::CriticalSection) -> usize {
        unsafe{ 
            *self.0.get()
        }
    }
}
unsafe impl Sync for DisplayCounter{}
static DISPLAY_COUNTER: DisplayCounter = DISPLAY_COUNTER_INIT;
```

Again, update the consumer code to match:

```rust
fn handle_timer1_1hz(mut hardware: Hardware) -> Hardware {
    // TODO: replace dummy crit sec when this is moved into a timer
    let cs = unsafe { avr_device::interrupt::CriticalSection::new() };
    DISPLAY_COUNTER.increment(&cs);
    let display_value = DISPLAY_COUNTER.value(&cs);
    ...
```

Build and deploy and verify this still works, and it does!

```bash
make clean build deploy
```

#### MultiSegmentLEDArray

Encapsulate the LED Array in a structure, and again use the UnsafeCell business:

```diff
/// ------------------------------------------------------------------------
/// MultiplexedLEDArray encapsulates the led array
/// ------------------------------------------------------------------------
struct MultiplexedLEDArray(UnsafeCell<[EightSegmentLEDCommonAnode; NUM_LEDS]>);

impl MultiplexedLEDArray {
    pub fn configure(&self, c: [usize; 8usize], a: [usize; NUM_LEDS], _cs: &avr_device::interrupt::CriticalSection) {
        // c: shared cathodes
        // a: anodes (not shared)
        unsafe {
            let mut leds = *self.0.get();
            for idx in 0..NUM_LEDS {
                leds[idx] = EightSegmentLEDCommonAnode::new([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], a[idx]]);
            }
            *self.0.get() = leds;
        }
    }
    pub fn led(&self, idx: usize, _cs: &avr_device::interrupt::CriticalSection) -> EightSegmentLEDCommonAnode {
        unsafe {
            let leds = *self.0.get();
            let led = leds[idx];
            led.clone()
        }
    }

    pub fn update(&self, idx: usize, data: u8, _cs: &avr_device::interrupt::CriticalSection) {
        unsafe {
            let mut leds = *self.0.get();
            leds[idx.clone()] = *leds[idx].update(data);
            *self.0.get() = leds;
        }
    }
}

unsafe impl Sync for MultiplexedLEDArray {}
static MULTIPLEXED_LED_ARRAY: MultiplexedLEDArray =
    MultiplexedLEDArray(
        UnsafeCell::new(
            arr![EightSegmentLEDCommonAnode{
                pins: [0,0,0,0,0,0,0,0,0],
                data: 0}; 2]));

```

To use it we have to configure it, otherwise, we just have zeros for the arduino gpio pins and nothing works:

```rust
#[arduino_uno::entry]
fn main() -> ! {
    unsafe {
        let cs = avr_device::interrupt::CriticalSection::new();
        MULTIPLEXED_LED_ARRAY.configure([2,3,4,5,6,7,8,9], [10, 11], &cs);
        // memory barrier to prevent instruction re-ordering
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

```

Some of the details in the led.rs changed, too:

```rust
#[derive(Clone,Copy,Debug)]
pub struct EightSegmentLEDCommonAnode {
    // pins (0-7 are A-DP, 8 is com)
    pub pins: [usize; 9],
    // data to display
    pub data: u8,
}

impl EightSegmentLEDCommonAnode {
    pub fn new(pins: [usize; 9]) -> EightSegmentLEDCommonAnode {
        EightSegmentLEDCommonAnode {
            pins,
            data: 0,
        }
    }
    pub fn default() -> EightSegmentLEDCommonAnode {
        EightSegmentLEDCommonAnode::new([0,1,2,3,4,5,6,7,8])
    }
    pub fn update(&mut self, data: u8) -> &EightSegmentLEDCommonAnode { self.data = data; self}
    pub fn data(&self) -> u8 { self.data.clone() }
    pub fn pins(&self) -> [usize; 9] { self.pins.clone() }
    pub fn com(&self) -> usize { self.pins[8]}
}
```

Build and deploy and verify this still works, and it does!

```bash
make clean build deploy
```


##### Dead Ends

The tricky thing about getting this far was how to initialize the static
(global) array for the array of leds. This is the code I'm speaking of:

```rust
/// ------------------------------------------------------------------------
/// MultiplexedLEDArray encapsulates the led array
/// ------------------------------------------------------------------------
struct MultiplexedLEDArray(UnsafeCell<[EightSegmentLEDCommonAnode; NUM_LEDS]>);
```

This bad boy needs to be a global, but OMG what a mess. I wanted this to be initialized as a const or at least a static variable so that it could be a global singleton. Mutations would be handled by the internal UnsafeCell as with all the other bits of state in this app. Heres what worked:

```rust
unsafe impl Sync for MultiplexedLEDArray {}
static MULTIPLEXED_LED_ARRAY: MultiplexedLEDArray =
    MultiplexedLEDArray(
        UnsafeCell::new(
            arr![EightSegmentLEDCommonAnode{
                pins: [0,0,0,0,0,0,0,0,0],
                data: 0}; 2]));
```


This is the inital value that basically carves out the space for the led array, but initialized to bogus values. Also note the WART... the size of the array is allocated as `2` because the const NUM_LEDS doesn't work in this macro. Bummerz.

Along the way, I experimented with some things that looked cool, but did not work for my use case:

###### array_init

[array_init](https://docs.rs/array-init/0.1.1/array_init/) is great for initializing arrays. I'll definitely use it all the time...just not for static or consts.


###### maybeinit

[maybeinit](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html) allows for having vars that start out uninitialized and they can get correctly filled out later. However, this results in a really ugly code pattern where you have to wrap access to the maybe init'd variable in an unsafe block for every access. I was hoping that there could be a barrier of some sort after which you could reason that the variable had, in fact been initialized... perhaps that's there and I just did not see it.



##### Live Ends

[arr_macro](https://lib.rs/crates/arr_macro) wound up solving for my use case, modulo the WART described above, and the fact that I have to re-initialize it in the conusming code.


#### Timers

Up to this point, all I've done is chew at the problem by wrapping the various bits of state in UnsafeCells and then re-deploying the app. But the point to all of this is to enable the ISRs for the timers and see if we deadlock or if we can actually run properly.

I'm hitting a blocking issue. I'll check in and ask some questions on the `avr-hal` project.

Question:

```rust
/// ------------------------------------------------------------------------
/// WrappedHardware encapsulates the arduino GPIO
/// ------------------------------------------------------------------------
struct Hardware {
    pub onboard_led_pin: Pin<mode::Output>,
    pub pins: [Pin<mode::Output>; 8usize + NUM_LEDS],
}

/*

XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
This does not work. The problem is...how do I create a default Output Pin
to populate this global/static variable? I can set it in the entrypoint,
but I need to allocate the space here, as I do for the other globals...

Specifically:

    Pin::<mode::Output>::new()

I don't see how to create a new generic output pin.
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

struct WrappedHardware(UnsafeCell<Hardware>);
unsafe impl Sync for WrappedHardware {}
static WRAPPED_HARDWARE: WrappedHardware =
    WrappedHardware(
        UnsafeCell::new(
            Hardware{
                    onboard_led_pin: Pin::<mode::Output>::new(),
                    pins: arr![Pin::<mode::Output>::new(); 10] }));

 */

```


### Summary

In the end, I could not get the ISRs to reliably `get` and `set` global data:

1. Sometimes I could not loop through pins within an ISR, as in:

    for x in 0..8 {
        pins[x].set_high().void_unwrap();
    }

But this _would_ work:

    pins[0].set_high().void_unwrap();
    pins[1].set_high().void_unwrap();
    pins[2].set_high().void_unwrap();
    ...

2. Sometimes setting global data would kill the ISR I was in, or it would kill
   the other ISR


3. If a pin was set high and I set it to high again, within an ISR, then this
   would kill the ISR. I had to resort to checking if the pin was already set
   high before setting it.

And it goes on and on and on.

Do yourself a favor. Do not use interrupts on the AVR.

## Links

* https://aminb.gitbooks.io/rust-for-c/content/arrays/index.html
* https://doc.rust-lang.org/reference/visibility-and-privacy.html
* https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html
* https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
* https://doc.rust-lang.org/std/primitive.array.html
* https://docs.rs/array-init/0.1.1/array_init/
* https://docs.rs/crate/avr-device/0.2.2/source/README.md
* https://docs.rust-embedded.org/book/concurrency/
* https://github.com/Rahix/avr-hal
* https://github.com/Rahix/avr-hal/issues/75#issuecomment-702675340
* https://stackoverflow.com/questions/23810032/how-to-specify-const-array-in-global-scope-in-rust
* https://upload.wikimedia.org/wikipedia/commons/c/c9/Pinout_of_ARDUINO_Board_and_ATMega328PU.svg
* https://users.rust-lang.org/t/initializing-an-array-of-structs/36586
* https://www.ameyalokare.com/rust/2017/11/02/rust-builder-pattern.html
* https://www.circuito.io/blog/arduino-uno-pinout/
* https://www.educative.io/edpresso/arrays-in-rust
* https://www.joshmcguigan.com/blog/array-initialization-rust/

## End

* [Prev](../07-multisegment-led-rust-arduino/readme.md)
* [Next]()
