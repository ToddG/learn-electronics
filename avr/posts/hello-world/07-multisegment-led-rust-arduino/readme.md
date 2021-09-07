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

But we can still do this, but go `old school`:

## Strategy (oldschoolz)

1. Create a working desktop app (targets x_86)
2. Add tests, etc., make it all work.
3. Create a skeleton avr app (targets avr)
4. Copy files from x_86 to avr project, file by file
5. Here's where it gets interesting
    a) Comment out stuff till things work
    b) Re-read the rust book, web links, etc.
    c) Ask questions on forums, write up tickets
    d) Simplify the code, stop using stuff from `std::`
    e) Use the led blinky light on the arduino extensively
    f) GOTO 4

Q: What happens if you have an array index error? Blinky don't blink.
A: There are a _lot_ of things the compiler allowed that would blow the
app up on the device. But if you have a blinky that's supposed to blink
and doesn't...well, you can use that for line by line troubleshooting. 
That's what I did.

## Code

### X_86

I won't go into the nitty gritty details of creating each file. [Look here for the code](./code/multisegment-led/linux):

```
code/multisegment-led/linux/
├── Cargo.lock
├── Cargo.toml
├── mseg-bin
├── mseg-lib
├── mseg-test
└── target

4 directories, 2 files
```

Project is wrapped up with a top-level workspace.

```
cargo build
cargo test
```


### AVR

I won't go into the nitty gritty details of creating each file. [Look here for the code](./code/multisegment-led/avr):

```
code/multisegment-led/avr/
├── avr-atmega328p.json
├── dump-avr-timers.py
├── Makefile
├── mseg-bin
├── mseg-lib
├── mseg-test
└── rust-toolchain

3 directories, 4 files
```

Things to note:

* dump-avr-timers.py : a nifty utility to help with specifying timers
* top level Makefile b/c embedded apps and cargo workspaces don't play well
  together
* the code morphed _a lot_ from what it looked like on the linux version
* turns out some of the ideas I had on the linux version (the hal for example)
  just didn't work out in the avr version

```
./dump-avr-timers.py 1 2 300

# ------------------------------------
# interrupt frequency: f
# prescaler: p
# compare match register value: cmr
# timers: t
# ------------------------------------
"f: 1, p: 1, cmr: 15999999.0, t: None"
"f: 1, p: 8, cmr: 1999999.0, t: None"
"f: 1, p: 64, cmr: 249999.0, t: None"
"f: 1, p: 256, cmr: 62499.0, t: [1]"
"f: 1, p: 1024, cmr: 15624.0, t: [1]"
"f: 2, p: 1, cmr: 7999999.0, t: None"
"f: 2, p: 8, cmr: 999999.0, t: None"
"f: 2, p: 64, cmr: 124999.0, t: None"
"f: 2, p: 256, cmr: 31249.0, t: [1]"
"f: 2, p: 1024, cmr: 7811.5, t: [1]"
"f: 300, p: 1, cmr: 53332.333333333336, t: [1]"
"f: 300, p: 8, cmr: 6665.666666666667, t: [1]"
"f: 300, p: 64, cmr: 832.3333333333334, t: [1]"
"f: 300, p: 256, cmr: 207.33333333333334, t: [0, 2]"
"f: 300, p: 1024, cmr: 51.083333333333336, t: [0, 2]"

```

To build and deploy:

```
make clean build deploy
```

I _could_ go full-circle and take learnings from the `avr` version and apply to the
`linux` version. Instead, I'll apply the learnings to the next project.

# Summary

[Watch Video](./countup-timer-in-rust.mp4)

## Links

This project required a _lot_ of reading. This is not the complete list:

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
* https://www.tutorialspoint.com/rust/rust_bitwise_operators.htm
* [The Rust Programming Language](https://nostarch.com/Rust2018)


Bugs
* https://github.com/japaric/heapless/issues/177
* https://github.com/Rahix/avr-hal/issues/71
* https://github.com/Rahix/avr-hal/issues/75
* https://stackoverflow.com/questions/63961435/rust-cargo-test-fails-for-arduino-targets-with-duplicate-lang-item-in-crate

## End

* [Prev](../06-blinky-led-rust-arduino/readme.md)
* [Next](../08-rust-timers/readme.md)
