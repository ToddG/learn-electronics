# 13 I2C Slave Driver

Notes on adding [I2C Slave capabilities](https://github.com/Rahix/avr-hal/issues/90) to [avr-hal](https://github.com/Rahix/avr-hal). This is needed to unblock [11-custom-i2c-driver](../11-custom-i2c-driver).

## Current Impl

NOTE: Since there are atmega328p chips in my UNO, that's the board I'll focus on as I figure this out.

I like to look at this as a simple transform:

    input -> macro -> output

So what's the input? What is the macro? And what does it output?

* input : [chips/atmega328p-hal/src/lib.rs](https://github.com/Rahix/avr-hal/chips/atmega328p-hal/src/lib.rs)
* macro : [avr-hal-generic/src/i2c.rs](https://github.com/Rahix/avr-hal/avr-hal-generic/src/i2c.rs).
* output : tbd

### Input

The input is `lib.rs` : [chips/atmega328p-hal/src/lib.rs](https://github.com/Rahix/avr-hal/chips/atmega328p-hal/src/lib.rs).

```rust
/// I2C Bus
pub mod i2c {
    use crate::port::portc;
    pub use avr_hal::i2c::*;

    avr_hal::impl_twi_i2c! {
        /// I2C based on ATmega328P's TWI peripheral
        pub struct I2c {
            peripheral: crate::atmega328p::TWI,
            pins: {
                sda: portc::PC4,
                scl: portc::PC5,
            },
            registers: {
                control: twcr {
                    enable: twen,
                    ack: twea,
                    int: twint,
                    start: twsta,
                    stop: twsto,
                },
                status: twsr {
                    prescaler: twps,
                    status: tws,
                },
                bitrate: twbr,
                data: twdr,
            },
        }
    }
}
```

### Macro

The macro is `i2c.rs` : [avr-hal-generic/src/i2c.rs](https://github.com/Rahix/avr-hal/avr-hal-generic/src/i2c.rs).

The `matcher` for this macro is :

```rust
/// Implement I2C traits for a TWI peripheral
#[macro_export]
macro_rules! impl_twi_i2c {
    (
        $(#[$i2c_attr:meta])*
        pub struct $I2c:ident {
            peripheral: $I2C:ty,
            pins: {
                sda: $sdamod:ident::$SDA:ident,
                scl: $sclmod:ident::$SCL:ident,
            },
            registers: {
                control: $twcr:ident {
                    enable: $twen:ident,
                    ack: $twea:ident,
                    int: $twint:ident,
                    start: $twstart:ident,
                    stop: $twstop:ident,
                },
                status: $twsr:ident {
                    prescaler: $twps:ident,
                    status: $tws:ident,
                },
                bitrate: $twbr:ident,
                data: $twdr:ident,
            },
        }
    ) => {

```

The transform generates a struct `I2c` and it's `impl` methods...here's my rough notes on the high level:

    I2c <I2cPullUp> -> new()
    I2c <I2cFloating> -> new_with_external_pullup()
    I2c <all> ->
        ping_slave()
        start()
        wait()
        transact()
        write_data()
        read_data()
        stop()


Then there's some blocking stuff...

    I2c <DelayMs> -> i2cdetect()
    I2c -> blocking Write()
    I2c -> blocking Read()
    I2c -> blocking WriteRead()


### Output

What does thes look like in it's expanded form?

    rustc -Z unstable-options --pretty expanded chips/atmega328p-hal/src/lib.rs


ERROR

    error[E0463]: can't find crate for `avr_hal_generic`
     --> chips/atmega328p-hal/src/lib.rs:3:1
      |
    3 | extern crate avr_hal_generic as avr_hal;
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0463`.

Ok, so let's try again:

    RUSTFLAGS="-Z unstable-options --pretty expanded" cargo build | tee build-expanded.txt

This generates some output and then errors out, too. I don't see the expanded macros in the output here.

Ok, moving on.

## Next Impl

From [notes](https://github.com/Rahix/avr-hal/issues/90):

```text
I'd say we define a second struct for the slave, similar to the existing one for the master:

$(#[$i2c_attr])*
pub struct $I2cSlave<CLOCK: $crate::clock::Clock, M> {
    p: $I2C,
    _clock: ::core::marker::PhantomData<CLOCK>,
    sda: $sdamod::$SDA<M>,
    scl: $sclmod::$SCL<M>,
}

Similar to the other one it will also need the two ::new() and ::new_with_external_pullup() constructors. Though here, the constructors need an additional argument which is the slave address. I think (but this needs to be checked) that we do not need the speed argument.

pub fn new(
    p: $I2C,
    sda: $sdamod::$SDA<$crate::port::mode::Input<$crate::port::mode::PullUp>>,
    scl: $sclmod::$SCL<$crate::port::mode::Input<$crate::port::mode::PullUp>>,
    slave_address: u8,
) -> $I2c<CLOCK, $crate::i2c::I2cPullUp> {
    // ...
}

For the rest of the API we'll first have to look deeper into the operation of the TWI peripheral.
```





```rust

```
