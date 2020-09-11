#include <Arduino.h>
#include <src/runnable.h>
#include <src/charactermap.h>
#include <src/led.h>
#include <src/ledwriter.h>

// Use Runnable to auto-wire components into 'loop'
Runnable *Runnable::headRunnable = NULL;

// Generic character map for an 8 digit LED
GenericEightSegmentLEDCharacterMap _genCharMap = GenericEightSegmentLEDCharacterMap();
GenericEightSegmentLEDCharacterMap *GenericEightSegmentLEDCharacterMap::characterMap = &_genCharMap;

// Wire an 8 segment led to arduino IO pins
EightSegmentLEDCommonAnode led = EightSegmentLEDCommonAnode(
        GenericEightSegmentLEDCharacterMap::characterMap,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9);

// This is a crappy little test harness to cycle through
// all of the mappings and display them on the LED
LedWriter gen = LedWriter(
        500,
        &led);

void setup() {
    Runnable::setupAll();
}

void loop() {
    Runnable::loopAll();
}
