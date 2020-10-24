// Written by Nick Gammon
// February 2012

#include <Wire.h>

const byte SLAVE_ADDRESS = 42;
const byte LED = 13;

void setup () 
  {
  Wire.begin ();
  pinMode (LED, OUTPUT);     
  }  // end of setup

void loop () 
  {
  for (byte x = 2; x <= 7; x++)
    {  
    Wire.beginTransmission (SLAVE_ADDRESS);
    Wire.write (x);
    if (Wire.endTransmission () == 0)
      digitalWrite (LED, HIGH); 
    else
      digitalWrite (LED, LOW); 
      
    delay (200);
    }  // end of for loop
  }  // end of loop
