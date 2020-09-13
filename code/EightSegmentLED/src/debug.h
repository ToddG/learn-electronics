//
// Created by todd on 9/12/20.
//

#ifndef EIGHTSEGMENTLED_DEBUG_H
#define EIGHTSEGMENTLED_DEBUG_H

#include <Arduino.h>
class Debug {
    bool _flip = false;
    uint8_t _level = 0;

public:
    static const uint8_t LEVEL_TRACE = 0;
    static const uint8_t LEVEL_DEBUG = 1;
    static const uint8_t LEVEL_INFO = 2;
    static const uint8_t LEVEL_ERROR = 3;

    Debug(uint8_t level): _level(level){
        pinMode(LED_BUILTIN, OUTPUT);
    }

    void level(uint8_t level){
        _level = level;
    }
    /**
     * Flip the onboard LED off and then on.
     * Prints out the state {HIGH|LOW} on serial console.
     */
    void flip() {
        this->debug("flip");
        _flip = !_flip;
        if (_flip) {
            this->trace("HIGH");
            digitalWrite(LED_BUILTIN, HIGH);
        } else {
            this->trace("LOW");
            digitalWrite(LED_BUILTIN, LOW);
        }
    }

    void trace(String msg){
        if(_level <= LEVEL_TRACE){
            Serial.print("TRACE: ");
            Serial.println(msg);
        }
    }

    void debug(String msg){
        if(_level <= LEVEL_DEBUG){
            Serial.print("DEBUG: ");
            Serial.println(msg);
        }
    }
    void info(String msg){
        if(_level <= LEVEL_INFO){
            Serial.print("INFO: ");
            Serial.println(msg);
        }
    }
    void error(String msg){
        if(_level <= LEVEL_ERROR) {
            Serial.print("ERROR: ");
            Serial.println(msg);
        }
    }
};

// global
Debug *debug = new Debug(Debug::LEVEL_DEBUG);

#endif //EIGHTSEGMENTLED_DEBUG_H
