//
// Created by todd on 9/11/20.
//

#ifndef EIGHTSEGMENTLED_GENERATOR_H
#define EIGHTSEGMENTLED_GENERATOR_H

#include <Arduino.h>
#include <src/counter.h>
#include <src/display.h>

class UpCountWriter {
public:
    void trigger() {
        _counter->increment();
        _counter->data(_buf);
        _display->set(_buf);
    }

    UpCountWriter(
            MultiCharacterUpCounter *counter,
            MultiCharacterLEDDisplay *display,
            uint8_t size
    ) :
            _counter(counter),
            _display(display),
            _size(size) {
        _buf = new uint8_t[_size];
        memset(_buf, 0, _size);
    };
private:
    MultiCharacterUpCounter *_counter;
    MultiCharacterLEDDisplay *_display;
    uint8_t _size;
    uint8_t *_buf;
};


#endif //EIGHTSEGMENTLED_GENERATOR_H
