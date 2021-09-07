#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum PinMode {
    INPUT,
    OUTPUT,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Pin {
    pub mode: PinMode,
    pub value: bool
}

impl Pin {
    pub fn new() -> Pin {
        Pin{
            mode: PinMode::OUTPUT,
            value: false
        }
    }
}

pub trait HAL {

    fn pin_mode(&mut self, pin_index: usize, mode: PinMode) -> ();
    fn digital_write(&mut self, pin_index: usize, data: bool) -> ();
    fn dump(&self)->Vec::<String>;
}

