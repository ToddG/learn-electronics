//
// Created by todd on 9/11/20.
//

#ifndef EIGHTSEGMENTLED_GENERATOR_H
#define EIGHTSEGMENTLED_GENERATOR_H

#include <Arduino.h>
#include "runnable.h"
#include "charactermap.h"
#include "led.h"

class LedWriter : Runnable {
    int sleep;
    EightSegmentLEDCommonAnode *led;

public:
    LedWriter(int sleep, EightSegmentLEDCommonAnode *led)
            : sleep(sleep), led(led) {}

private:
    void setup() {
    }

    void loop() {
        led->input = (led->input + 1) % led->characterMap->num_mappings();
        delay(sleep);
    }
};


#endif //EIGHTSEGMENTLED_GENERATOR_H
