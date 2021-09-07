#include <Wire.h>

void setup()
{
  Wire.begin();
  Serial.begin(9600, SERIAL_8N1);
  Serial.println("\nTEST SERIAL OUT\n");
} 
 
void loop()
{
   Serial.println("TEST: 123456789 ABCDEFG abcdefg");
   delay(1000);
}
