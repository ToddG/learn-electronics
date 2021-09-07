extern crate mseg_lib;
use mseg_lib::*;
use std::convert::TryInto;

fn main() {

    let x86_hal:Box<dyn hal::HAL> = platform::x86::new_hal(20);
    let mut led = led::new_eight_segment_led_common_anode(
        3,1,2,4,5,6,7,9,10, x86_hal);
    for d in led.dump(){println!{"{}", d}}
    println!("init...");
    led.init();
    for d in led.dump(){println!{"{}", d}}
    for i in 0..cmap::NUM_DISPLAY_ELEMENTS {
        led.set(i.try_into().unwrap());
        led.strobe();
        println!("set {} and strobe...", i);
        for d in led.dump(){println!{"{}", d}}
    }
}

//fn main() {
//    println!("Hello, world!");
//}
// fn main() {
//    let a:i32 = 2;     // Bit presentation 10
//    let b:i32 = 3;     // Bit presentation 11
//
//    let mut result:i32;
//
//    println!("--------------------------------");
//    println!("a => {} : {:b} : {:?}", a, a, a);
//    println!("b => {} : {:b} : {:?}", b, b, b);
//    println!("--------------------------------");
//
//    result = a & b;
//    println!("(a & b) => {}, {:b} ",result, result);
//
//    result = a | b;
//    println!("(a | b) => {}, {:b} ",result, result);
//
//    result = a ^ b;
//    println!("(a ^ b) => {}, {:b} ",result, result);
//
//    result = !b;
//    println!("(!b) => {}, {:b} ",result, result);
//
//    result = a << b;
//    println!("(a << b) => {}, {:b} ",result, result);
//
//    result = a >> b;
//    println!("(a >> b) => {}, {:b} ",result, result);
//
//    let mut input:usize;
//    let mut index:usize;
//    let mut bres:bool;
//
//    input = 1;
//    index = 1;
//    bres = mseg_lib::bits::get(input, index);
//    println!("(input, index, bres) => ({:b}, {}, {})", input, index, bres);
//
//    input = 0b11111111;
//    index = 0;
//    bres = mseg_lib::bits::get(input, index);
//    println!("(input, index, bres) => ({:b}, {}, {})", input, index, bres);
//    index = 1;
//    println!("(input, index, bres) => ({:b}, {}, {})", input, index, bres);
//
//    input = 0b00000000;
//
//    for i in 0..8 {
//       input = mseg_lib::bits::set(input, i);
//       bres = mseg_lib::bits::get(input, i);
//       println!("(input, index, bres) => ({:b}, {}, {})", input, i, bres);
//    }
//
// }
