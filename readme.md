# learn-electronics
Let's learn electronics together.

## Overview

Starting with the most basic circuits (and ICs), let's learn electronics via small projects and tutorials that we share here. To start, I'm approaching this with a series of 'hello world' tasks that become increasingly more interesting, see the Goals section below.

## QuickStart

### Phase 1

#### Posts

 * ![hello-world](posts/hello-world/01-multisegment-led)

#### Source

 * src/EightSegmentLED

## Dependencies

### Software

1. TODO

### Tools

#### Complete Lab

This is a more or less complete lab setup (you don't need all of this)

* https://www.youtube.com/watch?v=R_PbjbRaO2E

#### Cheap Lab

This is a much cheaper lab setup (you only need some of this)

* https://www.youtube.com/watch?v=HicV3Z6XLFA

#### Minimalist Lab

I think you could bootstrap without a scope, use an old wall wart for a power supply (or scavenge an old computer power supply), create a voltage divider with some resistors, and you are ready to go.

Things I think you need but you can go cheap on:

* soldering iron
* wire
* solder/flux
* breadboard
* perfboard
* multi-meters (2)
* capacitors
* resistors

#### No Lab

For the hello-world stuff, all you need is an arduino, a breadboard, some wires, a resistor, and an 8 segment LED. There are kits to be had for less than $25 that have all this and more.

### Books

The Art Of Electronics (old version, may cost ~$20-40)

* https://www.thriftbooks.com/w/the-art-of-electronics_paul-horowitz_winfield-hill/248534/#isbn=0521231515&idiq=4482077

Or get the latest, but $120 is pretty steep.

* https://www.adafruit.com/product/2356

Practical Electronics For Inventors

* https://www.thriftbooks.com/w/practical-electronics-for-inventors_paul-scherz/308734/#isbn=0071452818&idiq=8674669

I buy older books, then when I've read the whole thing (rarely but sometimes happens) I buy the new book.

### Chips

#### Phase 1 : Arduino ATMega328P

I bought a couple of these on ebay for about $10. Get the UNO R3 with socketed IC so you can remove the chip and embed it
on a board later. I also ordered 4 chips with resistor, oscillator, and socket for another $20.

* https://www.microchip.com/wwwproducts/en/ATMEGA328P

### Phase 2 : ESP32-S2

The dev kit is about $60 with shipping from digikey:

* https://docs.espressif.com/projects/esp-idf/en/latest/esp32s2/hw-reference/esp32s2/user-guide-esp32-s2-kaluga-1-kit.html
* https://www.digikey.com/products/en?keywords=ESP32-S2-Kaluga-1%20Kit

or we can use the STM Discovery board...

https://www.st.com/en/evaluation-tools/stm32f3discovery.html

I'm not sure which board to use for Phase 2 yet... suggestions?

### Phase 3

The F1C100s has 32MB in the chip, and can be had for $1.50 qty 1, less than $1 in quantity. I bought 10 of these for less than $20 including shipping.
According to the datasheet, this is an amazing piece of tech with loads of audio visual subsystems...

* https://www.thirtythreeforty.net/media/F1C100s_Datasheet_V1.0.pdf
* https://www.aliexpress.com/item/4001041333661.html?spm=a2g0o.productlist.0.0.6b7f1c73uAlR4v&algo_pvid=e7011709-5e10-48f4-847b-42632b183876&algo_expid=e7011709-5e10-48f4-847b-42632b183876-3&btsid=0bb0623615989711069561884e4897&ws_ab_test=searchweb0_0,searchweb201602_,searchweb201603_


## Goals

Output 'Hello World', or it's equivalent, using increasingly interesting and challenging technologies.

### Phase 1

1. Manually trigger an LED on and off, with a power supply, switch and a resistor.
1. Manually trigger the segments of a 7 segment LED display on and off, again using just power supply, switches, and resistors.
1. Use ATMega328P to programmatically control 7 segment LED display, using no libs, just the GPIO pins, power supply, and resistors. 
1. Solder a multi character LED display.
1. Controll multi character LED display directly via ATMega328P
1. Solder a SPI|I2C|UART backpack(s) for LED display
    Note: the backpack needs a micro-controller or a GPIO expander... options are MCP23017, PCF8574, or ATMega328P. Personally, I think the ATMega328P is the easiest choice as it already supports SPI|I2C|UART.
1. ATMega328P sends 'Hello World' over SPI|I2C|UART to backpack, backpack controls LED display

### Phase 2

Using the STM or ESP, output a graphical 'hello world' to a graphical display of some sort.

### Phase 3

Using the F1C100s, output 'hello world' using a video codec such as H.264,H.263,MPEG1/2/4 to a graphical display of some sort.

### Phase 4

Using the F1C100s, serve the 'hello world' video stream over the internet.


# Notes

I2C

* https://en.wikipedia.org/wiki/I%C2%B2C

STM

STM devices can speak many protocols: I²C, IrDA, SPI, UART/USART 

* https://www.digikey.com/product-detail/en/stmicroelectronics/STM8L151F3U6TR/497-14041-2-ND/3087877


This is the backpack that we are basically creating from scratch.

SparkFun 16x2 SerLCD - RGB Backlight (Qwiic)
https://www.sparkfun.com/products/16396

"""The on-board ATmega328P AVR microcontroller utilizes 11.0592 MHz crystal for greater communication accuracy with adjustable baud rates of 1200 through 1000000 but is default set at 9600. The firmware for this SerLCD is fully opensource and allows for any customizations you may need."""


Backpack chips

* https://www.instructables.com/id/DIY-I2C-LCD-Display-With-Inputs/
* https://circuitcrush.com/arduino/2015/11/02/diy-i2c-lcd.html
* http://electronics-diy.com/two-wire-i2c-arduino-lcd-display.php

Build up to E-Paper

* https://www.adafruit.com/?q=epaper

LED Matrix Display

* https://www.makerguides.com/max7219-led-dot-matrix-display-arduino-tutorial/
* https://www.maximintegrated.com/en/products/power/display-power-control/MAX7219.html/tb_tab3
* https://duckduckgo.com/?t=canonical&q=+MAX7219+led&ia=web

