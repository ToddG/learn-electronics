# Rust on Arduino Tutorial

I'm going to walk through a tutorial for deploying rust code to an
arduino. Spoiler, it didn't quite work, but with a little web searching
and some lucky guesses, I got it working.

## Tutorial

Following this tut:

* https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0

Links that unblocked me:

* https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
* https://os.phil-opp.com/freestanding-rust-binary/

## Commands


### Create

Create new app with cargo

    $ cargo new rust-arduino-blink

     Created binary (application) `rust-arduino-blink` package

What did that do?

    $ tree
    .
    ├── readme.md
    └── rust-arduino-blink
        ├── Cargo.toml
        └── src
            └── main.rs

    2 directories, 3 files

What's in those files?


    $ cat rust-arduino-blink/Cargo.toml
    [package]
    name = "rust-arduino-blink"
    version = "0.1.0"
    authors = ["..."]
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

and...

    $ cat rust-arduino-blink/src/main.rs
    fn main() {
        println!("Hello, world!");
    }

Ok, so we have a config and a 'hello world' app...


### Toolchain


#### Overrides

    rustup override set nightly

This does...

    info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
    info: latest update on 2020-09-13, rust version 1.48.0-nightly (dbb73f8f7 2020-09-12)
    info: downloading component 'cargo'
    info: downloading component 'clippy'
    info: downloading component 'rust-docs'
    info: downloading component 'rust-std'
     20.9 MiB /  20.9 MiB (100 %)  20.7 MiB/s in  1s ETA:  0s
    info: downloading component 'rustc'
     55.2 MiB /  55.2 MiB (100 %)  21.2 MiB/s in  2s ETA:  0s
    info: downloading component 'rustfmt'
    info: installing component 'cargo'
    info: Defaulting to 500.0 MiB unpack ram
    info: installing component 'clippy'
    info: installing component 'rust-docs'
     13.0 MiB /  13.0 MiB (100 %)   6.2 MiB/s in  1s ETA:  0s
    info: installing component 'rust-std'
     20.9 MiB /  20.9 MiB (100 %)   8.7 MiB/s in  2s ETA:  0s
    info: installing component 'rustc'
     55.2 MiB /  55.2 MiB (100 %)   9.5 MiB/s in  5s ETA:  0s
    info: installing component 'rustfmt'
    info: override toolchain for '/home/todd/repos/personal/learn-electronics/code/rust/rustOnArduinoUnoTutorial' set to 'nightly-x86_64-unknown-linux-gnu'

      nightly-x86_64-unknown-linux-gnu installed - rustc 1.48.0-nightly (dbb73f8f7 2020-09-12)

I wonder how I unset this later if I need to? Ah, the tut says:

    The above command overrides the toolchain of choice for only our current directory to be nightly.

Cool. But where is that change made? I don't see any changes in the local directory. Is this set in my local config?

    $ cat  ~/.rustup/settings.toml 
    default_toolchain = "stable"
    profile = "default"
    version = "12"

    [overrides]
    "/home/todd/repos/personal/learn-electronics/code/rust/rustOnArduinoUnoTutorial" = "nightly-x86_64-unknown-linux-gnu"

Well, how about that. Ok, so now we know where to delete the override later.


#### Pacman

The tut uses 'pacman' to install the cross compilation toolchain:

    pacman -S avr-gcc

Where 'pacman' is the package manager for their distro:

    I am on an arch linux distro (endeavour OS) where pacman is our package manager.

Since I'm on Ubuntu, I need to find and install avr-gcc? I'm not sure exactly, but here's what I think you need: 'arduino-core', 'gcc-avr', and 'avrdude':

    $ apt-cache search arduino | grep ^ardu
    arduino-core - Code, examples, and libraries for the Arduino platform

    $ dpkg -l | grep avr
    ii  avr-libc                                     1:2.0.0+Atmel3.6.0-1                             all          Standard C library for Atmel AVR development
    ii  avrdude                                      6.3-4                                            amd64        software for programming Atmel AVR microcontrollers
    ii  binutils-avr                                 2.26.20160125+Atmel3.6.0-1                       amd64        Binary utilities supporting Atmel's AVR targets
    ii  gcc-avr                                      1:5.4.0+Atmel3.6.0-1build1                       amd64        GNU C compiler (cross compiler for avr)

I think I'm set:

    $ which avrdude
    /usr/bin/avrdude
    $ which avr-gcc
    /usr/bin/avr-gcc


#### Dependencies

Here's the created Cargo.toml:

    [package]
    name = "rust-arduino-blink"
    version = "0.1.0"
    authors = ["..."]
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

Here's the mods we make to it:

    [package]
    name = "rust-arduino-blink"
    version = "0.1.0"
    authors = ["..."]
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    # A panic handler is needed.  This is a crate with the most basic one.
    panic-halt = "0.2.0"

    [dependencies.arduino-uno]
    git = "https://github.com/Rahix/avr-hal"
    [dependencies]

And we add more magic (see tut):

    $ tree -a
    .
    ├── original-tutorial.txt
    ├── readme.md
    └── rust-arduino-blink
        ├── avr-atmega328p.json
        ├── .cargo
        │   └── config.toml
        ├── Cargo.toml
        └── src
            └── main.rs

Notes: 

* the added file, 'avr-atmega328p.json' needs the '.json' at the end.


### Build

Build it! 

    $ cargo build
        Updating git repository `https://github.com/Rahix/avr-hal`
        Updating crates.io index
      Downloaded proc-macro-hack v0.5.18
      Downloaded vcell v0.1.2
      Downloaded nb v0.1.3
      Downloaded ufmt v0.1.0
      Downloaded embedded-hal v0.2.4
      Downloaded void v1.0.2
      Downloaded ufmt-write v0.1.0
      Downloaded semver v0.9.0
      Downloaded avr-device v0.2.2
      Downloaded avr-device-macros v0.2.2
      Downloaded bare-metal v0.2.5
      Downloaded panic-halt v0.2.0
      Downloaded paste v1.0.0
      Downloaded semver-parser v0.7.0
      Downloaded quote v1.0.7
      Downloaded unicode-xid v0.2.1
      Downloaded rustc_version v0.2.3
      Downloaded cfg-if v0.1.10
      Downloaded syn v1.0.40
      Downloaded nb v1.0.0
      Downloaded proc-macro2 v1.0.21
      Downloaded ufmt-macros v0.1.1
      Downloaded 22 crates (1.2 MB) in 0.59s
    error: "/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/Cargo.lock" does not exist, unable to build with the standard library, try:
            rustup component add rust-src

Ok... so let's do what it says...

    $ rustup component add rust-src
    info: downloading component 'rust-src'
    info: installing component 'rust-src'
    info: Defaulting to 500.0 MiB unpack ram


Now let's try building again...

    $ cargo build
      Downloaded adler v0.2.3
      Downloaded unicode-width v0.1.8
      Downloaded addr2line v0.13.0
      Downloaded miniz_oxide v0.4.0
      Downloaded hashbrown v0.9.0
      Downloaded compiler_builtins v0.1.35
      Downloaded libc v0.2.77
      Downloaded cc v1.0.58
      Downloaded getopts v0.2.21
      Downloaded rustc-demangle v0.1.16
      Downloaded object v0.20.0
      Downloaded gimli v0.22.0
      Downloaded 12 crates (1.8 MB) in 0.41s
       Compiling compiler_builtins v0.1.35
       Compiling core v0.0.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
       Compiling proc-macro2 v1.0.21
       Compiling unicode-xid v0.2.1
       Compiling syn v1.0.40
       Compiling semver-parser v0.7.0
       Compiling proc-macro-hack v0.5.18
       Compiling paste v1.0.0
       Compiling semver v0.9.0
       Compiling rustc_version v0.2.3
       Compiling quote v1.0.7
       Compiling bare-metal v0.2.5
       Compiling ufmt-macros v0.1.1
       Compiling avr-device-macros v0.2.2
       Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
       Compiling nb v1.0.0
       Compiling ufmt-write v0.1.0
       Compiling void v1.0.2
       Compiling cfg-if v0.1.10
       Compiling vcell v0.1.2
       Compiling panic-halt v0.2.0
       Compiling ufmt v0.1.0
       Compiling avr-device v0.2.2
       Compiling nb v0.1.3
       Compiling embedded-hal v0.2.4
       Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
       Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
       Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
       Compiling rust-arduino-blink v0.1.0 (/home/todd/repos/personal/learn-electronics/code/rust/rustOnArduinoUnoTutorial/rust-arduino-blink)
    error: language item required, but not found: `eh_personality`

    error: aborting due to previous error

    error: could not compile `rust-arduino-blink`

    To learn more, run the command again with --verbose.


So looking for 'eh_personality' shows some stuff here:

* https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
* https://os.phil-opp.com/freestanding-rust-binary/

The second link suggests that we need a more-betterer panic/abort thingy. So here i add these stanzas to Cargo.toml:

    [profile.dev]
    panic = "abort"

    [profile.release]
    panic = "abort"

Here's the full file:

    [package]
    name = "rust-arduino-blink"
    version = "0.1.0"
    authors = ["Todd Greenwood-Geer <pub+github@zwrob.com>"]
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    # A panic handler is needed.  This is a crate with the most basic one.
    panic-halt = "0.2.0"

    [dependencies.arduino-uno]
    git = "https://github.com/Rahix/avr-hal"

    [profile.dev]
    panic = "abort"

    [profile.release]
    panic = "abort"

Now try building...

    $ cargo build
       Compiling core v0.0.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
       Compiling rustc-std-workspace-core v1.99.0 (/home/todd/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
       Compiling compiler_builtins v0.1.35
       Compiling nb v1.0.0
       Compiling vcell v0.1.2
       Compiling ufmt-write v0.1.0
       Compiling cfg-if v0.1.10
       Compiling bare-metal v0.2.5
       Compiling void v1.0.2
       Compiling panic-halt v0.2.0
       Compiling ufmt v0.1.0
       Compiling avr-device v0.2.2
       Compiling nb v0.1.3
       Compiling embedded-hal v0.2.4
       Compiling avr-hal-generic v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
       Compiling atmega328p-hal v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
       Compiling arduino-uno v0.1.0 (https://github.com/Rahix/avr-hal#ad6fedd3)
       Compiling rust-arduino-blink v0.1.0 (/home/todd/repos/personal/learn-electronics/code/rust/rustOnArduinoUnoTutorial/rust-arduino-blink)
        Finished dev [unoptimized + debuginfo] target(s) in 23.26s

Success! Does it run?

### Run

Figure out where your arduino is showing up, for me it's /dev/ttyUSB0:


    $ ls -lsa /dev/ttyUSB0 
    0 crw-rw---- 1 root dialout 188, 0 Sep 13 10:53 /dev/ttyUSB0

Make sure you can write to it, even though I'm a member of dialout, I cannot
access this w/o the following chmod:

    $ sudo chmod 777 /dev/ttyUSB0 
    $ ls -lsa /dev/ttyUSB0 
    0 crwxrwxrwx 1 root dialout 188, 0 Sep 13 10:53 /dev/ttyUSB0

Now flash the device with the codez:

    $ avrdude -patmega328p -carduino -P/dev/ttyUSB0 -D "-Uflash:w:target/avr-atmega328p/debug/rust-arduino-blink.elf:e"

    avrdude: AVR device initialized and ready to accept instructions

    Reading | ################################################## | 100% 0.00s

    avrdude: Device signature = 0x1e950f (probably m328p)
    avrdude: reading input file "target/avr-atmega328p/debug/rust-arduino-blink.elf"
    avrdude: writing flash (10418 bytes):

    Writing | ################################################## | 100% 1.71s

    avrdude: 10418 bytes of flash written
    avrdude: verifying flash memory against target/avr-atmega328p/debug/rust-arduino-blink.elf:
    avrdude: load data flash data from input file target/avr-atmega328p/debug/rust-arduino-blink.elf:
    avrdude: input file target/avr-atmega328p/debug/rust-arduino-blink.elf contains 10418 bytes
    avrdude: reading on-chip flash data:

    Reading | ################################################## | 100% 1.37s

    avrdude: verifying ...
    avrdude: 10418 bytes of flash verified

    avrdude: safemode: Fuses OK (E:00, H:00, L:00)

    avrdude done.  Thank you.

You'll have to trust me. The led is blinking!


