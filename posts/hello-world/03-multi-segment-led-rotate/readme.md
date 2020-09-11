# 03 Spinning LED

Cycle through all of the LED segments and light them up one at a time. The only trick here is that I'm using a common *anode* LED. I'll be using the UNO's digital IO pins 2-9 to control the LED segments... However, because this is a common *anode* LED, I need to send the positive rail to the common anode, and then sink the various LED's to ground (without shorting them out :-)). So, I throw the ~200Ohm resistor off the UNO's 3.3V positive rail, and then sink the LED's to ground via the LOW setting on each digital IO pin to light up that LED segment. Question, is LOW really ground on the UNO?

## Build

### Wiring

    (UNO) 3.3V -> (R) 200Ohm -> (LED) anode
                                (LED) 1,2,4,5,6,7,9 
                                    -> (UNO) Digital IO 2-9

### Sketch

    // Set pins HIGH to start as we set a pin to LOW
    // to turn on an LED segment (sinking to ground)
    void setup() {
      for(int i = 2;i<=9;i++){
        pinMode(i, OUTPUT);
        digitalWrite(i, HIGH);
      }
    }

    void loop() {
      for(int i = 2;i<=9;i++){
        digitalWrite(i, LOW);
        delay(1000);
        digitalWrite(i, HIGH);
      }
    }

## Success

Apparently setting the Digital IO pin to LOW is equivalent to setting it to GROUND. Cool.

[Spinning LED](resources/03-spinning-led-thumb2.mp4)

Done.

## Notes

I used ffmpeg to downsample the massive .mp4 my phone generated:
(https://askubuntu.com/questions/1163156/how-to-resize-and-convert-mov-to-mp4-on-ubuntu-16-04)

     ffmpeg -i 03-spinning-led.mp4 -r 10 -s 400x400 03-spinning-led-thumb2.mp4

##  End

* [Prev](../02-multisegment-led-arduino-uno/readme.md)
* [Next](../04-multi-segment-characters/readme.md)
