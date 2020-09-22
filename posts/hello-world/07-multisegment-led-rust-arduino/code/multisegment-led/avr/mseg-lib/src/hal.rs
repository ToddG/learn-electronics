pub mod avr_hal {
    #[cfg(cpu="atmega328p")]
    impl HAL for Platform {
        fn pin_mode(&self, pin: u8, mode: PinMode) -> () {
            // noop
        }

        fn pin_write(&self, pin: u8, data: bool) -> () {
            // noop
        }
    }
}
