/// HAL (hardware abstraction layer) can be implemented for different platforms

pub mod x86 {
    use crate::hal::*;

    #[derive(Debug)]
    pub struct PlatformX86 {
        pins: Vec<Pin>
    }

    impl HAL for PlatformX86 {
        fn pin_mode(&mut self, pin_index: usize, pin_mode: PinMode) -> () {
            let mut p = self.pins[pin_index];
            p.mode = pin_mode;
            self.pins[pin_index] = p;
        }

        fn digital_write(&mut self, pin_index: usize, pin_value: bool) -> () {
            let mut p = self.pins[pin_index];
            p.value = pin_value;
            self.pins[pin_index] = p;
        }

        fn dump(&self) -> Vec::<String> {
            let mut v: Vec::<String> = Vec::new();
            let mut i = 0;
            for p in &self.pins {
                v.push(format!("pin[{}]: mode: {:?}, value: {:?}", i, p.mode, p.value));
                i += 1;
            }
            v
        }
    }

    pub fn new_hal(num_pins: usize) -> Box<dyn HAL> {
        let v = std::iter::repeat_with(|| Pin::new())
            .take(num_pins)
            .collect::<Vec<_>>();
        Box::new(PlatformX86 { pins: v })
    }
}
