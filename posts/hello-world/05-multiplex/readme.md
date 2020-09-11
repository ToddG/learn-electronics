# 05 Multiplex

(INPROC)

There must be a way to drive more than one LED from a single arduino.
I remember watching a video where the LED's on a display were blinking
or sort of unreadable b/c of the refresh rate colliding with the frame
rate on the recording. So I suspect that we can time slice driving the
LEDs...A quick test proves this is true. So what if used hooked multiple
LEDs up to the same negative pins (2-9) but multiplex the positive rail 
with some of the other arduino digital IO pins?

A quick search shows that I am treading on well traveled ground. This thing
I just described is called "Charlieplexing":

* https://circuitdigest.com/microcontroller-projects/charlieplexing-arduino-to-control-12-leds-with-4-gpio-pins

## Sketch 1 : use an IO port for positive rail


## Links

* https://sites.google.com/site/qeewiki/books/avr-guide/common-timer-theory
* https://hackaday.com/2017/05/05/using-modern-c-techniques-with-arduino/
* https://www.instructables.com/id/LED-Multiplexing-101-6-and-16-RGB-LEDs-With-Just-a/
* http://www.righto.com/2009/07/secrets-of-arduino-pwm.html
* https://www.arduino.cc/en/Tutorial/Foundations
* https://www.arduino.cc/en/Main/Standalone
* https://www.arduino.cc/en/Guide/BoardAnatomy
* https://www.arduino.cc/en/Tutorial/Sketch
* https://www.arduino.cc/en/Tutorial/DigitalPins
* https://www.arduino.cc/en/Tutorial/AnalogInputPins
* https://www.arduino.cc/en/Tutorial/PWM
* https://www.arduino.cc/en/Tutorial/Memory
* https://www.arduino.cc/reference/en/language/variables/utilities/progmem/
* https://www.arduino.cc/en/Reference/EEPROM
* https://www.arduino.cc/en/Main/Standalone
* https://www.arduino.cc/en/Hacking/HomePage
* https://www.arduino.cc/reference/en/
* https://www.arduino.cc/en/Tutorial/Variables
* https://www.arduino.cc/en/Reference/FunctionDeclaration
* https://www.arduino.cc/en/Tutorial/VidorHDL
* https://www.arduino.cc/en/Hacking/LibraryTutorial
* https://sites.google.com/site/qeewiki/books/avr-guide/timers-on-the-atmega328
* https://sites.google.com/site/qeewiki/books/avr-guide/common-timer-theory
* https://sites.google.com/site/qeewiki/datasheet-library
* https://sites.google.com/site/qeewiki/friends-tutorials/hexskews-atmega32l-bootloader
* https://www.instructables.com/id/Arduino-Timer-Interrupts/
* https://www.arduino.cc/en/Reference/PortManipulation

## End

* [Prev](../04a-code-structure/readme.md)
* [Next]()
