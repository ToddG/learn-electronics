// Written by Nick Gammon
// February 2012

#include <Wire.h>

const byte MY_ADDRESS = 42;

void setup () 
  {
  Wire.begin (MY_ADDRESS);
  for (byte i = 2; i <= 7; i++)
    pinMode (i, OUTPUT);
  // set up receive handler
  Wire.onReceive (receiveEvent);
  }  // end of setup

void loop() 
  {
  // nothing in main loop
  }

// called by interrupt service routine when incoming data arrives
void receiveEvent (int howMany)
  {
  for (int i = 0; i < howMany; i++)
    {
    byte c = Wire.read ();
    // toggle requested LED
    if (digitalRead (c) == LOW)
      digitalWrite (c, HIGH);
    else
      digitalWrite (c, LOW);
    }  // end of for loop
}  // end of receiveEvent
