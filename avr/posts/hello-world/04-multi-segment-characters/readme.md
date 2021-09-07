# 04 Characters

## Goal

Create a library to parse an input and display characters on the 8 segment led:

    Output 0,1,2,(and error, which is all segments lit up)

## Build

Same as 03.

## Preliminaries

Now it's time to start looking at the language spec:

[Sketch language Ref](https://www.arduino.cc/reference/en/)

Ok, I have a ton of questions about the language now, passing by reference vs. by value,
as well as what is the proper design pattern here? Is this C? C++? WTF? And here I stumble
upon this post:

[ArduinoTheOOWay](https://paulmurraycbr.github.io/ArduinoTheOOWay.html)

And I am stoked, because he is proposing a sane way to use OO in the exact way
that my all time fave, Yegor, does here:

[Elegant Objects](https://www.yegor256.com/elegant-objects.html)

And now that we are starting to write real code, it's time to switch to a
*real* editor, one that supports refactoring, code completion, etc. _CLION_ is
my choice (I have a personal license for the entire jetbrains tool suite):

* [CLION](https://plugins.jetbrains.com/clion)
* [CLION Arduino Plugin](https://plugins.jetbrains.com/plugin/7889-arduino)

And now that we are using CLion, it's probably time to understand the make system
that it is using, under the hood: [CMake](https://cmake.org):

* [CMake Tutorial](https://cmake.org/cmake/help/latest/guide/tutorial/index.html)
* [Build Tools Compared](https://medium.com/@julienjorge/an-overview-of-build-systems-mostly-for-c-projects-ac9931494444)

CLion is executing the following command:

    $ cmake -DCMAKE_BUILD_TYPE=Debug -G "CodeBlocks - Unix Makefiles" src/EightSegmentLED

And this generates:

    -- Generating EightSegmentLED
    -- Using /usr/bin/avr-objcopy for converting firmware image to hex
    -- ARDUINO Boards:
    --              uno: Arduino Uno
    --        atmega328: Arduino Duemilanove w/ ATmega328
    --        diecimila: Arduino Diecimila or Duemilanove w/ ATmega168
    --          nano328: Arduino Nano w/ ATmega328
    --             nano: Arduino Nano w/ ATmega168
    --         mega2560: Arduino Mega 2560 or Mega ADK
    --             mega: Arduino Mega (ATmega1280)
    --         leonardo: Arduino Leonardo
    --          esplora: Arduino Esplora
    --            micro: Arduino Micro
    --          mini328: Arduino Mini w/ ATmega328
    --             mini: Arduino Mini w/ ATmega168
    --         ethernet: Arduino Ethernet
    --              fio: Arduino Fio
    --            bt328: Arduino BT w/ ATmega328
    --               bt: Arduino BT w/ ATmega168
    --       LilyPadUSB: LilyPad Arduino USB
    --       lilypad328: LilyPad Arduino w/ ATmega328
    --          lilypad: LilyPad Arduino w/ ATmega168
    --         pro5v328: Arduino Pro or Pro Mini (5V, 16 MHz) w/ ATmega328
    --            pro5v: Arduino Pro or Pro Mini (5V, 16 MHz) w/ ATmega168
    --           pro328: Arduino Pro or Pro Mini (3.3V, 8 MHz) w/ ATmega328
    --              pro: Arduino Pro or Pro Mini (3.3V, 8 MHz) w/ ATmega168
    --        atmega168: Arduino NG or older w/ ATmega168
    --          atmega8: Arduino NG or older w/ ATmega8
    --     robotControl: Arduino Robot Control
    --       robotMotor: Arduino Robot Motor
    --
    -- ARDUINO Programmers:
    --         avrisp: AVR ISP
    --     avrispmkii: AVRISP mkII
    --     usbtinyisp: USBtinyISP
    --         usbasp: USBasp
    --       parallel: Parallel Programmer
    --     arduinoisp: Arduino as ISP
    --
    -- Configuring done
    -- Generating done
    -- Build files have been written to: learn-electronics


CLion and Arduino Plugin Notes: 

* I have to manually connect to the arduino each time you start the IDE (why doesn't it automatically reconnect?)
* once connected, there is a run profile that regex's 'upload' to deploy to the device
* this plugin is kindof raw, but it does work

## Sketch

[Code](resources/eight_segment_led.ino)

The key things about this sketch are:

* Use of the [Runnable pattern](https://paulmurraycbr.github.io/ArduinoTheOOWay.html) for wiring stuff up
* Use of Abstract Base Classes (ABC) for polymorphism
* Use of constructor injection (aka Elegant Objects) for composing objects

The program flow is :

1. A Generator class creates inputs and writes them to the LED class (EightSegmentLEDCommonAnode)
1. The LED class delegates to a CharacterMap to map the input to an 8 segment display
1. The LED class maps the 8 segment display to the digital IO pins on the arduino
1. The arduino pins drive the physical LED

## Success

[Running](resources/04-sequenced-led.mp4)

## Notes

A future post will cover running RUST on the AVR. Here's latest post I've found on the topic (2020):

[RUST](https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0)

## End

* [Prev](../03-multi-segment-led-rotate/readme.md)
* [Next](../04a-code-structure/readme.md)
