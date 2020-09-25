# 07 Multisegment LED using Rust + Arduino UNO

## Rust + Testing

Posts 01-05 really made me think about testing code. It's a real time sink to
deploy code to a device. These devices are tiny. And kinda slow. And getting
any real testing on them is going to eat up all the space we need for the
binaries that we want to deploy. So, how can we structure things so that we can
easily write _and_ test the code? In Chapter 14 (page 297) of [The Rust
Programming Language](https://nostarch.com/Rust2018) they talk about Cargo
Workspaces. I bet we could create a workspace and create separate crates for
the different use cases of the code:

|CODE|DEV ENV|TARGET DEVICE|NOTES|
|----|-------|-------------|-----|
|test|0|X|only runs on the dev environment|
|bin|X|0|only runs on target device|
|lib|X|X|invoked and tested by test code, and runs on target device|

Well, we could except that I ran into weird bugs...

* workspaces don't work well with embedded projects
* embedded stuff doesn't use `std::` which is pervasive in rust apps 

But we can still do this.

## Source

[code](./code/multisegment-led)

## Steps

* Create a project that targets the default OS (linux, whatever)
* Get the code working there, with tests, etc.
* Create a second project that targets the AVR
* Copy the code into the AVR project
* Do whatever we have to in order to get things to compile on the AVR 

Yeah, I should probably figure out how to cross compile, but there's a lot of
weird annotations, like `#[no_std]` and I'm not sure how to properly IFDEF
these annotations so that I can cross compile the `lib` crate. So let's KISS
and see how far we get...after all, this is _supposed_ to be about learning
electronics...

### Create (linux) Project

Make the project dir and the crates:

```
mkdir -p code/multisegment-led/linux
cd code/multisegment-led/linux
cargo new --bin mseg-bin
cargo new --bin mseg-lib
cargo new --bin mseg-test
```

Create the top level cargo file, Cargo.toml:

```
[workspace]

members = [
    "mseg-bin",
    "mseg-lib",
    "mseg-test",
]
```

Here's what we have:

```
.
├── Cargo.lock
├── Cargo.toml
├── mseg-bin
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── mseg-lib
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── mseg-test
    ├── Cargo.toml
    └── src
        └── lib.rs
```

Now let's make sure that both mseg-bin and mseg-test depend on mseg-lib. Add
the `mseg-lib` dependency to each of their Cargo.toml files.

```
[dependencies]
mseg-lib = { path = "../mseg-lib" }
```

Make sure this builds.

```
$ cargo build
   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/linux/mseg-lib)
   Compiling mseg-test v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/linux/mseg-test)
   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/linux/mseg-bin)
    Finished dev [unoptimized + debuginfo] target(s) in 0.71s
```

#### Bit twidling

And now for a brief departure. I know from the previous C/C++ version of this
project that I'll need to be able to twiddle bits. So let's explore that now
and add some tests for this.

I'll start with this post:

* https://www.tutorialspoint.com/rust/rust_bitwise_operators.htm

So, taking that example as a starting point here's what I came up with:

* Add the above code to the `msg-bin/src/main.rs` and understand how bit fields work.
* Add a quick little `mod` to `mseg-lib` that gets and sets bit fields
* Add some tests for the above

Here's what that looks like:

`msg-bin/src/main.rs`

```
extern crate mseg_lib;

fn main() {
   let a:i32 = 2;     // Bit presentation 10
   let b:i32 = 3;     // Bit presentation 11

   let mut result:i32;
   
   println!("--------------------------------");
   println!("a => {} : {:b} : {:?}", a, a, a);
   println!("b => {} : {:b} : {:?}", b, b, b);
   println!("--------------------------------");

   result = a & b;
   println!("(a & b) => {}, {:b} ",result, result);

   result = a | b;
   println!("(a | b) => {}, {:b} ",result, result);

   result = a ^ b;
   println!("(a ^ b) => {}, {:b} ",result, result);

   result = !b;
   println!("(!b) => {}, {:b} ",result, result);

   result = a << b;
   println!("(a << b) => {}, {:b} ",result, result);

   result = a >> b;
   println!("(a >> b) => {}, {:b} ",result, result);

   let mut input:u8;
   let mut index:u8;
   let mut bres:bool;

   input = 1;
   index = 1;
   bres = mseg_lib::bits::get(input, index);
   println!("(input, index, bres) => ({:b}, {}, {})", input, index, bres);

   input = 0b11111111;
   index = 0;
   bres = mseg_lib::bits::get(input, index);
   println!("(input, index, bres) => ({:b}, {}, {})", input, index, bres);
   index = 1;
   println!("(input, index, bres) => ({:b}, {}, {})", input, index, bres);

   input = 0b00000000;

   for i in 0..8 {
      input = mseg_lib::bits::set(input, i);
      bres = mseg_lib::bits::get(input, i);
      println!("(input, index, bres) => ({:b}, {}, {})", input, i, bres);
   }
}
```

`mseg-lib/src/lib.rs`
```
pub mod bits {

    pub fn set(input:u8, index:u8) -> u8 {
        let mask:u8 = 1 << index;
        input | mask
    }
    pub fn get(input:u8, index:u8) -> bool {
        let mask:u8 = 1 << index;
        let mut result = input & mask;
        result >>= index;
        return if result == 1 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_set(){
        let b: u8 = 0b00000000;

        for i in 0..8 {
            for j in 0..8 {
                let actual = bits::get(bits::set(b, i), j);
                if i == j {
                    assert_eq!(true, actual);
                }else{
                    assert_eq!(false, actual);
                }
            }
        }
    }
}
```

And a test run in `mseg-lib` looks like:

```
$ cd mseg-lib
$ cargo test

   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/linux/mseg-lib)
    Finished test [unoptimized + debuginfo] target(s) in 0.59s
     Running /home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/linux/target/debug/deps/mseg_lib-d3f07cb06a782eb7

running 2 tests
test tests::it_works ... ok
test tests::test_bits_set ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests mseg-lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```
#### Linux implementation

Here's what I came up with:

```
.
├── Cargo.lock
├── Cargo.toml
├── mseg-bin
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── mseg-lib
│   ├── cargo-test.txt
│   ├── Cargo.toml
│   └── src
│       ├── bits.rs
│       ├── cmap.rs
│       ├── hal.rs
│       ├── led.rs
│       ├── lib.rs
│       └── platform.rs
├── mseg-test
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── test.out

6 directories, 15 files
```

The key part is that I created a hardware abstraction layer (hal) that can be
implemented either by my desktop (x86) or by an arduino (avr). I'm not sure how
helpful this will be, but it's been cool to develop against as I can run
_almost_ the exact code that's going to run on the avr here on my
desktop...modulo whatever fun and games I discover porting...

#### Add tests

Make sure tests pass.

```
$ cargo test

    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running target/debug/deps/mseg_bin-c2a8a2fe449d6726

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/mseg_lib-d3f07cb06a782eb7

running 2 tests
test bits::tests::test_bits_set ... ok
test tests::it_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/mseg_test-2100ad5588352acd

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests mseg-lib

running 4 tests
test src/led.rs - led::new_eight_segment_led_common_anode (line 42) ... ok
test src/bits.rs - bits::get (line 44) ... ok
test src/bits.rs - bits::set (line 15) ... ok
test src/cmap.rs - cmap::segments (line 172) ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests mseg-test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```


### Create AVR Project

Here, I create the `avr` project. Then I copy, file by file or code snippet by
code snippet, the code from the `linux` implementation over here. Due to
missing `std::` I suspect this will be interesting.

TODO NEXT


## Links
* https://articles.bchlr.de/traits-dynamic-dispatch-upcasting
* https://book.avr-rust.com/002.1-installing-required-third-party-tools.html
* https://dev.to/itnext/rust-basics-structs-methods-and-traits-3p64
* https://doc.rust-lang.org/book/appendix-02-operators.html
* https://doc.rust-lang.org/book/ch05-01-defining-structs.html
* https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits
* https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
* https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
* https://doc.rust-lang.org/cargo/getting-started/first-steps.html
* https://doc.rust-lang.org/cargo/reference/profiles.html
* https://doc.rust-lang.org/cargo/reference/profiles.html#overrides
* https://doc.rust-lang.org/nomicon/panic-handler.html
* https://doc.rust-lang.org/reference/conditional-compilation.html
* https://doc.rust-lang.org/reference/type-layout.html
* https://doc.rust-lang.org/rust-by-example/flow_control/for.html
* https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
* https://doc.rust-lang.org/rust-by-example/trait.html
* https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
* https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
* https://doc.rust-lang.org/std/keyword.dyn.html
* https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
* https://docs.rust-embedded.org/book/collections/index.html
* https://docs.rust-embedded.org/discovery/02-requirements/index.html
* https://droogmic.github.io/microrust/
* https://fasterthanli.me/series/making-our-own-executable-packer
* https://fasterthanli.me/series/making-our-own-executable-packer/part-12
* https://github.com/avr-rust/
* https://github.com/avr-rust/ruduino
* https://immunant.com/blog/2020/01/bitfields/
* https://joshleeb.com/blog/rust-traits-trait-objects/
* https://k155la3.blog/2020/03/21/learning-embedded-rust-by-building-riscv-powered-robot-part-1/
* https://lib.rs/no-std
* https://matematikaadit.github.io/posts/rust-turbofish.html
* https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
* https://medium.com/swlh/rust-structs-options-and-traits-485eecd9c718
* https://os.phil-opp.com/freestanding-rust-binary/
* https://rust-embedded.github.io/book/intro/index.html
* https://stackoverflow.com/questions/36061560/can-i-take-a-byte-array-and-deserialize-it-into-a-struct
* https://stackoverflow.com/questions/37843379/is-it-possible-to-use-box-with-no-std
* https://stackoverflow.com/questions/38896155/what-is-the-bitwise-not-operator-in-rust
* https://stackoverflow.com/questions/40467995/how-do-you-set-clear-and-toggle-a-single-bit-in-rust
* [The Rust Programming Language](https://nostarch.com/Rust2018)


Bugs
* https://stackoverflow.com/questions/63961435/rust-cargo-test-fails-for-arduino-targets-with-duplicate-lang-item-in-crate
* https://github.com/Rahix/avr-hal/issues/71
* https://github.com/japaric/heapless/issues/177

## End

* [Prev](../06-blinky-led-rust-arduino/readme.md)
* [Next]()
