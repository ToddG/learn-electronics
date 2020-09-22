extern crate mseg_lib;

#[cfg(test)]
mod test_led {

    #[test]
    fn test_foo(){
        assert_eq!(42, mseg_lib::led::foo());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
