#include <Arduino.h>
#include <src/debug.h>
#include <src/counter.h>
#include <src/timers.h>
#include <src/tests.h>
#include <src/charactermap.h>
#include <src/led.h>
#include <src/display.h>
#include <src/writer.h>


// Generic character map for an 8 digit LED
GenericEightSegmentLEDCharacterMap _genCharMap = GenericEightSegmentLEDCharacterMap();
GenericEightSegmentLEDCharacterMap *GenericEightSegmentLEDCharacterMap::characterMap = &_genCharMap;

// Wire an 8 segment led to arduino IO pins
EightSegmentLEDCommonAnode led1 = EightSegmentLEDCommonAnode(
        GenericEightSegmentLEDCharacterMap::characterMap,
        10,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9);

EightSegmentLEDCommonAnode led2 = EightSegmentLEDCommonAnode(
        GenericEightSegmentLEDCharacterMap::characterMap,
        11,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9);

EightSegmentLEDCommonAnode ledArray[] = {led1, led2};
MultiCharacterLEDDisplay *display = new MultiCharacterLEDDisplay(ledArray, 2);
MultiCharacterUpCounter *counter =
        new MultiCharacterUpCounter(2, GenericEightSegmentLEDCharacterMap::num_segments());
UpCountWriter *writer = new UpCountWriter(
        counter,
        display,
        2
        );

void setup() {
    Serial.begin(115200);
    configure_timer_0();
    configure_timer_1();
    sei();//allow interrupts
    run_tests();
}


void loop() {
    // Nothing
}

/**
 * Configured at 2kHz
 */
ISR(TIMER0_COMPA_vect) {
    led1.strobe();
    led2.strobe();
}
/**
 * Configured at 1Hz
 */
ISR(TIMER1_COMPA_vect) {
    Serial.println("Version: 003");
    debug->flip();
    writer->trigger();
}