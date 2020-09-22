# bugs

Extracts of things I tried and that _should_ work, but don't. I'm 
pulling these dead-ends out of the readme and putting them here.

Basically, there are 2 issues:

* workspaces don't work with the arduino targets
* cargo tests don't work with arduino targets (no_std)

Links

* https://stackoverflow.com/questions/63961435/rust-cargo-test-fails-for-arduino-targets-with-duplicate-lang-item-in-crate
* https://os.phil-opp.com/testing/
* https://github.com/rust-lang/cargo/issues/7359
* https://github.com/japaric/utest
* https://os.phil-opp.com/minimal-rust-kernel/#set-a-default-target
* https://os.phil-opp.com/minimal-rust-kernel/#using-cargo-run
* https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#choo-choo-release-channels-and-riding-the-trains
* https://github.com/japaric/rust-cross

### Create Workspace

Create directory:

```bash
mkdir -p code/multisegment-led
```

Create workspace file:

```bash
cd code/multisegment
vim Cargo.toml
```

Cargo.toml
```toml
[workspace]

members = [
    "mseg-lib",
    "mseg-bin",
    "mseg-test"
]
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
tree
```

Output...
```bash
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

Build:

```bash
cargo build
```

Output...
```bash
Compiling mseg-test v0.1.0 (multisegment-led/mseg-test)
Compiling mseg-bin v0.1.0 (multisegment-led/mseg-bin)
Compiling mseg-lib v0.1.0 (multisegment-led/mseg-lib)
Finished dev [unoptimized + debuginfo] target(s) in 3.56s
```

### Add Testing Support

Make *mseg-test* depend on the *mseg-lib* and start adding code to *mseg-lib*
and tests to *mseg-test*. Make *mseg-bin* depend on *mseg-lib* and then deploy
*mseg-bin* to the device. The idea here is to test the tool-chain to make sure
that we can:

* deploy code to the device (and run it there)
* test the code locally on the dev environment

#### Blinky Revisited

So let's start by bringing in the code from
[06-blinky-led-rust-arduino](../06-blinky-led-rust-arduino) and see if we can
get that running in this toolchain first.

Configure *mseg-bin* to use the nightly toolchain:

```bash
$ cd mseg-bin
~/multisegment-led/mseg-bin $ rustup override set nightly
info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
info: override toolchain for '/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin' set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.48.0-nightly (dbb73f8f7 2020-09-12)
```

Add dependencies to the `mseg-bin/Cargo.toml`:

```toml
[dependencies]
# A panic handler is needed.  This is a crate with the most basic one.
panic-halt = "0.2.0"

[dependencies.arduino-uno]
git = "https://github.com/Rahix/avr-hal"
```

Add the avr-atmega328p.json file as `mseg-bin/avr-atmega328p.json`:

```bash
$ ../06-blinky-led-rust-arduino/code/rustOnArduinoUnoTutorial/rust-arduino-blink/avr-atmega328p.json code/multisegment-led/mseg-bin/.
```

For review, this is the avr-atmega328p.json file:

```json
{
    "llvm-target": "avr-unknown-unknown",
    "cpu": "atmega328p",
    "target-endian": "little",
    "target-pointer-width": "16",
    "target-c-int-width": "16",
    "os": "unknown",
    "target-env": "",
    "target-vendor": "unknown",
    "arch": "avr",
    "data-layout": "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8",

    "executables": true,

    "linker": "avr-gcc",
    "linker-flavor": "gcc",
    "pre-link-args": {
      "gcc": ["-Os", "-mmcu=atmega328p"]
    },
    "exe-suffix": ".elf",
    "post-link-args": {
      "gcc": ["-Wl,--gc-sections"]
    },

    "singlethread": false,
    "no-builtins": false,

    "no-default-libraries": false,

    "eh-frame-header": false
  }
```

Build (remember we are in multisegment-led/mseg-bin):

```bash
cargo build
```

Output...
```bash
    Updating git repository `https://github.com/Rahix/avr-hal`
    Updating crates.io index
  Downloaded paste v1.0.1
  Downloaded syn v1.0.41
  Downloaded 2 crates (241.5 KB) in 0.82s
   Compiling proc-macro2 v1.0.21
   Compiling unicode-xid v0.2.1
   Compiling semver-parser v0.7.0
   Compiling syn v1.0.41
   Compiling proc-macro-hack v0.5.18
   Compiling nb v1.0.0
   Compiling cfg-if v0.1.10
   Compiling vcell v0.1.2
   Compiling void v1.0.2
   Compiling ufmt-write v0.1.0
   Compiling paste v1.0.1
   Compiling panic-halt v0.2.0
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.7
   Compiling avr-device-macros v0.2.2
   Compiling ufmt-macros v0.1.1
   Compiling avr-device v0.2.2
   Compiling ufmt v0.1.0
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin)
    Finished dev [unoptimized + debuginfo] target(s) in 34.27s
```

Wow! It built, no probs. Wait a minute. That was way too easy. What did I forget? Ah, the codez.

Here's our `mseg-bin/src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

But we want the blinky led thingy from the 06 tut. So update `mseg-bin/src/main.rs` to be:

```rust
#![no_std]
#![no_main]
extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 25);
    }
}
```

Now build.

```bash
cargo build

   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin)
error: Ensure that you are using an AVR target! You may need to change directories or pass a --target flag to cargo. See
       https://github.com/Rahix/avr-device/pull/41 for more details.
  --> mseg-bin/src/main.rs:15:1
   |
15 | #[arduino_uno::entry]
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: language item required, but not found: `eh_personality`

error: aborting due to 2 previous errors

error: could not compile `mseg-bin`

To learn more, run the command again with --verbose.
```

Now that's what we are expecting, per the 06 tuturial. So let's add the magic panic handlers to the `mseg-bin/Cargo.toml`

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

Will it build?

```bash
~/multisegment-led/mseg-bin $ cargo build

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin/Cargo.toml
workspace: /home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/Cargo.toml
   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin)
error: Ensure that you are using an AVR target! You may need to change directories or pass a --target flag to cargo. See
       https://github.com/Rahix/avr-device/pull/41 for more details.
  --> mseg-bin/src/main.rs:15:1
   |
15 | #[arduino_uno::entry]
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: language item required, but not found: `eh_personality`

error: aborting due to 2 previous errors

error: could not compile `mseg-bin`

To learn more, run the command again with --verbose.

```

The tubes show that this should have been fixed in Rust 1.43, and I'm using 1.48:

* https://stackoverflow.com/questions/45794917/specific-profiles-for-workspace-members

Ah, perhaps they mean that I have to move this `profile` stuff into the root `Cargo.toml`? Let's try that...

multisegment-led/Cargo.toml
```toml
[workspace]

members = [
    "mseg-lib",
    "mseg-bin",
    "mseg-test"
]

[profile.dev.package.mseg-bin]
panic = "abort"

[profile.release.package.mseg-bin]
panic = "abort"
```

multisegment-led/mseg-bin/Cargo.toml
```toml
[package]
name = "mseg-bin"
version = "0.1.0"
authors = ["Todd Greenwood-Geer <pub+github@zwrob.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# A panic handler is needed.  This is a crate with the most basic one.
panic-halt = "0.2.0"

[dependencies.arduino-uno]
git = "https://github.com/Rahix/avr-hal"
```

Ok, so build...

```bash
~/multisegment-led/mseg-bin $ cargo build
error: failed to parse manifest at `/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/Cargo.toml`

Caused by:
  `panic` may not be specified in a `package` profile
```

Sigh, go back and re-read the 06 tutorial...

*Don't forget to reference the avr config in the .cargo/config.toml:*

Oops. I forgot that.

```bash
mkdir .cargo
vim .cargo/config.toml
```

.cargo/config.toml
```
[build]
target = "avr-atmega328p.json"

[unstable]
build-std = ["core"]
```

Now try building again...

```bash
$ cargo build
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin/Cargo.toml
workspace: /home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/Cargo.toml
   Compiling compiler_builtins v0.1.35
   Compiling core v0.0.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling bare-metal v0.2.5
   Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling nb v1.0.0
   Compiling cfg-if v0.1.10
   Compiling vcell v0.1.2
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
   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin)
error: language item required, but not found: `eh_personality`

error: aborting due to previous error

error: could not compile `mseg-bin`

To learn more, run the command again with --verbose.
```

Ok, so now we can build, but we still need a way to specify the panic handlers
for the dev and release profiles. Now, the curious george in me is wondering... why does this magic hidden
`.cargo/config.toml` even exist? Sure seems like it's a hack to override stuff for `arduino`. So let's
copy the panic stuff in there and see if it works... So now the config files are:

multisegment-led/Cargo.toml
```toml
[workspace]

members = [
    "mseg-lib",
    "mseg-bin",
    "mseg-test"
]
```


multisegment-led/mseg-bin/Cargo.toml
```toml
[package]
name = "mseg-bin"
version = "0.1.0"
authors = ["Todd Greenwood-Geer <pub+github@zwrob.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# A panic handler is needed.  This is a crate with the most basic one.
panic-halt = "0.2.0"

[dependencies.arduino-uno]
git = "https://github.com/Rahix/avr-hal"
```


multisegment-led/mseg-bin/.cargo/config.toml
```toml
[build]
target = "avr-atmega328p.json"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[unstable]
build-std = ["core"]
```

Now try to build...

```bash
 cargo build
   Compiling core v0.0.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling compiler_builtins v0.1.35
   Compiling nb v1.0.0
   Compiling ufmt-write v0.1.0
   Compiling void v1.0.2
   Compiling vcell v0.1.2
   Compiling cfg-if v0.1.10
   Compiling bare-metal v0.2.5
   Compiling panic-halt v0.2.0
   Compiling ufmt v0.1.0
   Compiling avr-device v0.2.2
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin)
    Finished dev [unoptimized + debuginfo] target(s) in 24.81s
```

Success!!! Well almost... Let's move up to the parent `workspace` directory and build from there.

```bash
cd ..
pwd
/home/todd/multisegment-led
```

```bash
cargo build
```

Output
```
 cargo build
   Compiling nb v1.0.0
   Compiling cfg-if v0.1.10
   Compiling void v1.0.2
   Compiling ufmt-write v0.1.0
   Compiling vcell v0.1.2
   Compiling panic-halt v0.2.0
   Compiling mseg-test v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-test)
   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-lib)
   Compiling ufmt v0.1.0
   Compiling bare-metal v0.2.5
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling avr-device v0.2.2
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-bin v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-bin)
error: Ensure that you are using an AVR target! You may need to change directories or pass a --target flag to cargo. See
       https://github.com/Rahix/avr-device/pull/41 for more details.
  --> mseg-bin/src/main.rs:15:1
   |
15 | #[arduino_uno::entry]
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: language item required, but not found: `eh_personality`

error: aborting due to 2 previous errors

error: could not compile `mseg-bin`

To learn more, run the command again with --verbose.
```

OMG. I am going insane. Actually, the nightmare continued for a few more hours,
and eventually I decided that, as near as I can tell, `workspaces` are broken
for use with arduino devices. So it's time to retreat and formulate a new plan.
That plan is pretty simple:

* delete workspaces
* get each crate building
* add dependencies
    * bin depends on lib
    * test depends on lib
* make sure stuff still builds
* get the tests to run and build

For this, we move to the next section...


#### Make a test

*mseg-lib*

```bash
$ cat mseg-lib/Cargo.toml 
[package]
name = "mseg-lib"
version = "0.1.0"
authors = ["Todd Greenwood-Geer <pub+github@zwrob.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# A panic handler is needed.  This is a crate with the most basic one.
panic-halt = "0.2.0"

[dependencies.arduino-uno]
git = "https://github.com/Rahix/avr-hal"


~/multisegment-led $ cat mseg-lib/.cargo/config.toml 
[build]
target = "../avr-atmega328p.json"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[unstable]
build-std = ["core"]

~/multisegment-led $ cat mseg-lib/src/lib.rs
#![no_std]
extern crate panic_halt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Note the:

    #![no_std]
    extern crate panic_halt;


*mseg-bin*

```bash
~/multisegment-led $ cat mseg-bin/Cargo.toml 
[package]
name = "mseg-bin"
version = "0.1.0"
authors = ["Todd Greenwood-Geer <pub+github@zwrob.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# A panic handler is needed.  This is a crate with the most basic one.
panic-halt = "0.2.0"
arduino-uno = { git = "https://github.com/Rahix/avr-hal" }
mseg-lib = { path = "../mseg-lib" }

$ cat mseg-bin/.cargo/config.toml 
[build]
target = "../avr-atmega328p.json"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[unstable]
build-std = ["core"]

$ cat mseg-bin/src/main.rs
#![no_std]
#![no_main]
extern crate panic_halt;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 25);
    }
}

```

Note the:

    #![no_std]
    #![no_main]

    and

    #[arduino_uno::entry]


*mseg-test*

```bash
~/multisegment-led $ cat mseg-test/Cargo.toml
[package]
name = "mseg-test"
version = "0.1.0"
authors = ["Todd Greenwood-Geer <pub+github@zwrob.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

mseg-lib = { path = "../mseg-lib" }


~/multisegment-led $ cat mseg-test/src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```


Note that `mseg-test` doesn't exclude the standard rust libraries.

Now, for one Makefile to rule them all...

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

.PHONY: test
test:
	(cd mseg-test && cargo test)

.PHONY: deploy
deploy:
	(cd mseg-bin && avrdude -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/mseg-bin.elf:e")
```

```bash
$ make clean build test
(cd mseg-lib && cargo clean)
(cd mseg-bin && cargo clean)
(cd mseg-test && cargo clean)
(cd mseg-lib && cargo build)
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
   Compiling quote v1.0.7
   Compiling bare-metal v0.2.5
   Compiling ufmt-macros v0.1.1
   Compiling avr-device-macros v0.2.2
   Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling nb v1.0.0
   Compiling vcell v0.1.2
   Compiling void v1.0.2
   Compiling ufmt-write v0.1.0
   Compiling cfg-if v0.1.10
   Compiling panic-halt v0.2.0
   Compiling ufmt v0.1.0
   Compiling avr-device v0.2.2
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-lib)
    Finished dev [unoptimized + debuginfo] target(s) in 32.53s
(cd mseg-bin && cargo build)
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
   Compiling cfg-if v0.1.10
   Compiling void v1.0.2
   Compiling ufmt-write v0.1.0
   Compiling vcell v0.1.2
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
    Finished dev [unoptimized + debuginfo] target(s) in 33.50s
(cd mseg-test && cargo build)
   Compiling proc-macro2 v1.0.21
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.41
   Compiling semver-parser v0.7.0
   Compiling nb v1.0.0
   Compiling proc-macro-hack v0.5.18
   Compiling cfg-if v0.1.10
   Compiling void v1.0.2
   Compiling ufmt-write v0.1.0
   Compiling vcell v0.1.2
   Compiling paste v1.0.1
   Compiling panic-halt v0.2.0
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.7
   Compiling avr-device-macros v0.2.2
   Compiling ufmt-macros v0.1.1
   Compiling avr-device v0.2.2
   Compiling ufmt v0.1.0
   Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
   Compiling mseg-lib v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-lib)
   Compiling mseg-test v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-test)
    Finished dev [unoptimized + debuginfo] target(s) in 25.87s
(cd mseg-test && cargo test)
   Compiling mseg-test v0.1.0 (/home/todd/repos/personal/learn-electronics/posts/hello-world/07-multisegment-led-rust-arduino/code/multisegment-led/mseg-test)
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running target/debug/deps/mseg_test-5dab2b7d7d378fa2

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests mseg-test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

##### Add a function and test it

What a drag. Ok, so it seems that `cargo test` does not work with arduino libraries?


```bash
cd mseg-test
cargo test
```

[errors](./mseg-test/cargo-test-errors.txt)

As well, I cannot get the avr-hal lib to pass it's tests, either:

https://github.com/Rahix/avr-hal/issues/71

I hate to admit defeat. I think I'll see if there's a rust chat somewhere that I can post to...
