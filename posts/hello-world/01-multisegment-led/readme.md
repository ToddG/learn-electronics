# 01 Multisegment LED

## Goal

Light up one of the LED segments.

## Components

### LED

![LED Image](resources/sm4105w6-led.jpeg)

I don't have my printer hooked up, but here's what I think are the specs for this LED:

![https://www.velleman.eu/downloads/29/infosheets/vmp502_sma42056etc.pdf](https://www.velleman.eu/downloads/29/infosheets/vmp502_sma42056etc.pdf)

I've also downloaded the spec here:

![SM4105WS LED SPEC](resources/vmp502_sma42056etc.pdf)

And since I haven't hooked up my printer yet, here are my notes from the spec:

![Notes](resources/led-notes.jpeg)

### Build

Starting with the ususal suspects...

![the-ususal-suspects.jpeg](resources/the-ususal-suspects.jpeg)

From the spec, we have:

    FV = 2.5V
    IF = 25mA

So a bit of math gives:

    V = IR
    R = V/I = 2.4V/0.020A = 120 Ohms

So I whacked in a 220Ohm resistor, set my PS to 2.4V and limited the current to 25mA and we have light!

![success](resources/led-is-on.jpeg)

## End

* ![Prev](../../../readme.md)
* ![Next](../02-multisegment-led-arduino-uno/readme.md)
