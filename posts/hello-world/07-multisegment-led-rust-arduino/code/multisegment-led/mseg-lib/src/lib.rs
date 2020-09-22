#![no_std]
extern crate panic_halt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod foo{
    pub fn return_bar() -> &'static str{
        "bar"
    }
}
