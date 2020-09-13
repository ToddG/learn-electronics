//
// Created by todd on 9/11/20.
//

#ifndef EIGHTSEGMENTLED_DISPLAY_H
#define EIGHTSEGMENTLED_DISPLAY_H

#include <Arduino.h>
#include "led.h"

class MultiCharacterLEDDisplay {
public:
    MultiCharacterLEDDisplay(
            EightSegmentLEDCommonAnode ledArray[],
            uint8_t size) :
            _ledArray(ledArray),
            _size(size) {
    }

    void set(uint8_t *input) {
        for (uint8_t i = 0; i < _size; i++) {
            _ledArray[i].set(input[i]);
        }
    }

private:
    uint8_t _size;
    EightSegmentLEDCommonAnode *_ledArray;
};

#endif //EIGHTSEGMENTLED_DISPLAY_H
