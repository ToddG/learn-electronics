# 12 Rust Macros

I've started working on adding [I2C Slave capabilities](https://github.com/Rahix/avr-hal/issues/90) to [avr-hal](https://github.com/Rahix/avr-hal).

This means I need to learn about macros...

## Macros

https://doc.rust-lang.org/reference/macros-by-example.html

### Baby steps

* create a simple macro
* dump the expanded macro
* compile it
* run it

#### Create app

    cargo new my-macros
    cargo build
    cargo run

#### Procedural Macro

Expand the default hello world app:

    rustc -Z unstable-options --pretty expanded main.rs


```rust
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;

fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Hello World\n"],
                                                         &match () {
                                                              () => [],
                                                          }));
    };
}
```

Replace the 'hello world' with a macro. See:  https://medium.com/@phoomparin/a-beginners-guide-to-rust-macros-5c75594498f1 :

```rust
macro_rules! yo {
    ($name:expr) => {
        println!("Yo {}!", $name);
    };
}

fn main() {
    yo!("world")
}
```

Build...

```bash
cargo build
cargo run
```

Outputs:

        Finished dev [unoptimized + debuginfo] target(s) in 0.01s
         Running `target/debug/my-macros`
    Yo world!

What's the macro actually expand to? We could run... 

    rustc -Z unstable-options --pretty expanded main.rs

...but first we have to configure rust to use the nightly toolchain:

    echo "nightly" > rust-toolchain

Now we can expand this:

    rustc -Z unstable-options --pretty expanded main.rs


```rust
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
macro_rules! yo { ($ name : expr) => { println ! ("Yo {}!", $ name) ; } ; }

fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Yo ", "!\n"],
             &match (&"world",) {
                  (arg0,) =>
                  [::core::fmt::ArgumentV1::new(arg0,
                                                ::core::fmt::Display::fmt)],
              }));
    }
}
```

What's the difference between the original 'hello world' app and the app with a macro?

First app:

```rust
::std::io::_print(
    ::core::fmt::Arguments::new_v1(
        &["Hello World\n"], 
        &match () { 
            () => [], }));

```

vs. the app with a macro:

```rust
::std::io::_print(
    ::core::fmt::Arguments::new_v1(
        &["Yo ", "!\n"],
        &match (&"world",) {
            (arg0,) =>
            [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt)], }));
```

Uhoh... more unanswered questions...

* Q: Where is `new_v1` defined?
* Q: How does this `match` work?
* Q: How does `world` get inserted?

NOTE: It'd be cool to show an example of swapping out the concrete class for
something within a macro. I think there's an example of this in my Links
section below...I forget. But the idea would be to do what Bryan Cantrill did
[here](http://dtrace.org/blogs/bmc/) where he swapped out a HashMap for a
BTreeMap when testing performance on something:

    -    rects: HashMap<u64, RefCell>, // rectangles for this entity
    +    rects: BTreeMap<u64, RefCell>, // rectangles for this entity


There are also [procedural
macros](https://doc.rust-lang.org/reference/procedural-macros.html): function,
derive, and attribute macros.

I'll check those out later.

## Links

### Macros
* https://danielkeep.github.io/practical-intro-to-macros.html
* https://danielkeep.github.io/tlborm/book/index.html
* https://doc.rust-lang.org/1.7.0/reference.html#macros
* https://doc.rust-lang.org/book/ch19-06-macros.html
* https://doc.rust-lang.org/book/macros.html
* https://doc.rust-lang.org/cargo/index.html
* https://doc.rust-lang.org/edition-guide/editions/creating-a-new-project.html
* https://doc.rust-lang.org/error-index.html
* https://doc.rust-lang.org/nightly/unstable-book/index.html
* https://doc.rust-lang.org/nomicon/index.html
* https://doc.rust-lang.org/reference/conditional-compilation.html
* https://doc.rust-lang.org/reference/index.html
* https://doc.rust-lang.org/reference/macros-by-example.html
* https://doc.rust-lang.org/rustc/index.html
* https://doc.rust-lang.org/rustdoc/index.html
* https://doc.rust-lang.org/stable/embedded-book/
* https://doc.rust-lang.org/std/index.html
* https://github.com/celaus/rust-macros/blob/master/src/lib.rs
* https://medium.com/@phoomparin/a-beginners-guide-to-rust-macros-5c75594498f1
* https://rust-cli.github.io/book/index.html
* https://rustwasm.github.io/docs/book/why-rust-and-webassembly.html
* https://www.rust-lang.org/learn
* https://stackoverflow.com/questions/38040327/how-to-pass-rustc-flags-to-cargo
* http://dtrace.org/blogs/bmc/
