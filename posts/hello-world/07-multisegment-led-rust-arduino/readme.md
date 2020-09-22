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

## Source

[code](./code/multisegment-led)

## Steps

* Create a project that targets the default OS (linux, whatever)
* Get the code working here, with tests, etc.
* Create a project that targets the AVR
* Copy the code into the AVR project

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


#### Add tests

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

Additional Links:
* https://immunant.com/blog/2020/01/bitfields/
* https://doc.rust-lang.org/reference/type-layout.html
* https://stackoverflow.com/questions/36061560/can-i-take-a-byte-array-and-deserialize-it-into-a-struct
* https://stackoverflow.com/questions/38896155/what-is-the-bitwise-not-operator-in-rust
* https://stackoverflow.com/questions/40467995/how-do-you-set-clear-and-toggle-a-single-bit-in-rust
* https://doc.rust-lang.org/book/appendix-02-operators.html
* https://doc.rust-lang.org/rust-by-example/flow_control/for.html





### Create (AVR) Project (INPROC)
=======
This path is [littered with broken glass.](./bugs.md)

### Create Project
>>>>>>> 65f4bf8... WIP: hello-world/07-multisegment-led-rust-arduino

```
mkdir -p code/multisegment-led
cd code/multisegment-led
```

### Create crates

Create mseg-bin:

```bash
cargo new --bin mseg-bin
```

Output...
```bash
    warning: compiling this new crate may not work due to invalid workspace configuration

    failed to read `multisegment-led/mseg-lib/Cargo.toml`

    Caused by:
      No such file or directory (os error 2)
         Created binary (application) `mseg-bin` package
```

Create mseg-lib:

```bash
cargo new --lib mseg-lib
```

Output...
```bash
warning: compiling this new crate may not work due to invalid workspace configuration

failed to read `multisegment-led/mseg-test/Cargo.toml`

Caused by:
  No such file or directory (os error 2)
     Created library `mseg-lib` package
```

Create mseg-test:

```bash
cargo new --lib mseg-test
```

Output...
```bash
Created library `mseg-test` package
```

Directories:

```bash
tree -a
```

Output...
```bash
.
├── avr-atmega328p.json
├── .gitignore
├── Makefile
├── mseg-bin
│   ├── .cargo
│   │   └── config.toml
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── mseg-lib
│   ├── .cargo
│   │   └── config.toml
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── mseg-test
    ├── .cargo
    │   └── config.toml
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── lib.rs
```

Makefile:

```make

.PHONY: help
help:
	# -----------------------------------------------------------------------------
	# Targets:
	#
	#	clean 		: run `cargo clean` for each crate
	#	help 		: show this message
	#	build		: run `cargo build` for each crate
	#	test		: run `cargo test` for the mseg-test crate
	#	deploy		: use `avrdude` to deploy mseg-bin to device
	#
	# end.
	# -----------------------------------------------------------------------------


.PHONY: clean
clean:
	(cd mseg-lib && cargo clean)
	(cd mseg-bin && cargo clean)
	(cd mseg-test && cargo clean)

.PHONY: build
build:
	(cd mseg-lib && cargo build)
	(cd mseg-bin && cargo build)
	(cd mseg-test && cargo build)

.PHONY: deploy
deploy:
	(cd mseg-bin && avrdude -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/mseg-bin.elf:e")
```

Build

```
make clean build deploy
```

Output...
```
(cd mseg-lib && cargo clean)
(cd mseg-bin && cargo clean)
(cd mseg-test && cargo clean)
(cd mseg-lib && cargo build)
   Compiling compiler_builtins v0.1.35
   Compiling core v0.0.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling proc-macro2 v1.0.21
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.41
   Compiling semver-parser v0.7.0
   Compiling proc-macro-hack v0.5.18
   Compiling paste v1.0.1
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling quote v1.0.7
   Compiling bare-metal v0.2.5
   Compiling ufmt-macros v0.1.1
   Compiling avr-device-macros v0.2.2
   Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling nb v1.0.0
   Compiling vcell v0.1.2
   Compiling void v1.0.2
   Compiling cfg-if v0.1.10
   Compiling ufmt-write v0.1.0
   Compiling panic-halt v0.2.0
   Compiling ufmt v0.1.0
   Compiling avr-device v0.2.2
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-lib)
    Finished dev [unoptimized + debuginfo] target(s) in 32.39s
(cd mseg-bin && cargo build)
   Compiling compiler_builtins v0.1.35
   Compiling core v0.0.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling proc-macro2 v1.0.21
   Compiling unicode-xid v0.2.1
   Compiling semver-parser v0.7.0
   Compiling syn v1.0.41
   Compiling proc-macro-hack v0.5.18
   Compiling paste v1.0.1
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.7
   Compiling avr-device-macros v0.2.2
   Compiling ufmt-macros v0.1.1
   Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling nb v1.0.0
   Compiling vcell v0.1.2
   Compiling void v1.0.2
   Compiling cfg-if v0.1.10
   Compiling ufmt-write v0.1.0
   Compiling panic-halt v0.2.0
   Compiling ufmt v0.1.0
   Compiling avr-device v0.2.2
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-lib)
   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin)
    Finished dev [unoptimized + debuginfo] target(s) in 33.71s
(cd mseg-test && cargo build)
    Updating crates.io index
   Compiling compiler_builtins v0.1.35
   Compiling core v0.0.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling proc-macro2 v1.0.21
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.41
   Compiling semver-parser v0.7.0
   Compiling proc-macro-hack v0.5.18
   Compiling paste v1.0.1
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.7
   Compiling avr-device-macros v0.2.2
   Compiling ufmt-macros v0.1.1
   Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling nb v1.0.0
   Compiling vcell v0.1.2
   Compiling cfg-if v0.1.10
   Compiling void v1.0.2
   Compiling ufmt-write v0.1.0
   Compiling panic-halt v0.2.0
   Compiling ufmt v0.1.0
   Compiling avr-device v0.2.2
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-lib)
   Compiling mseg-test v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-test)
    Finished dev [unoptimized + debuginfo] target(s) in 34.18s
(cd mseg-bin && avrdude -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/mseg-bin.elf:e")

avrdude: AVR device initialized and ready to accept instructions

Reading | ################################################## | 100% 0.12s

avrdude: Device signature = 0x1e950f (probably m328p)
avrdude: reading input file "target/avr-atmega328p/debug/mseg-bin.elf"
avrdude: writing flash (10418 bytes):

Writing | ################################################## | 100% 1.68s

avrdude: 10418 bytes of flash written
avrdude: verifying flash memory against target/avr-atmega328p/debug/mseg-bin.elf:
avrdude: load data flash data from input file target/avr-atmega328p/debug/mseg-bin.elf:
avrdude: input file target/avr-atmega328p/debug/mseg-bin.elf contains 10418 bytes
avrdude: reading on-chip flash data:

Reading | ################################################## | 100% 1.34s

avrdude: verifying ...
avrdude: 10418 bytes of flash verified

avrdude: safemode: Fuses OK (E:00, H:00, L:00)

avrdude done.  Thank you.
```

### Create the code for displaying multisegment led

TODO: this is next


## Links
* https://articles.bchlr.de/traits-dynamic-dispatch-upcasting
* https://book.avr-rust.com/002.1-installing-required-third-party-tools.html
* https://dev.to/itnext/rust-basics-structs-methods-and-traits-3p64
* https://doc.rust-lang.org/book/ch05-01-defining-structs.html
* https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits
* https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
* https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
* https://doc.rust-lang.org/cargo/getting-started/first-steps.html
* https://doc.rust-lang.org/cargo/reference/profiles.html
* https://doc.rust-lang.org/cargo/reference/profiles.html#overrides
* https://doc.rust-lang.org/nomicon/panic-handler.html
* https://doc.rust-lang.org/reference/conditional-compilation.html
* https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
* https://doc.rust-lang.org/rust-by-example/trait.html
* https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
* https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
* https://doc.rust-lang.org/std/keyword.dyn.html
* https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
* https://docs.rust-embedded.org/discovery/02-requirements/index.html
* https://droogmic.github.io/microrust/
* https://fasterthanli.me/series/making-our-own-executable-packer
* https://fasterthanli.me/series/making-our-own-executable-packer/part-12
* https://github.com/avr-rust/
* https://github.com/avr-rust/ruduino
* https://joshleeb.com/blog/rust-traits-trait-objects/
* https://lib.rs/no-std
* https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
* https://medium.com/swlh/rust-structs-options-and-traits-485eecd9c718
* https://os.phil-opp.com/freestanding-rust-binary/
* https://rust-embedded.github.io/book/intro/index.html
* https://stackoverflow.com/questions/37843379/is-it-possible-to-use-box-with-no-std
* [The Rust Programming Language](https://nostarch.com/Rust2018)


Bugs
* https://stackoverflow.com/questions/63961435/rust-cargo-test-fails-for-arduino-targets-with-duplicate-lang-item-in-crate
* https://github.com/Rahix/avr-hal/issues/71

## End

* [Prev](../06-blinky-led-rust-arduino/readme.md)
* [Next]()
