//
// Created by todd on 9/12/20.
//

#ifndef EIGHTSEGMENTLED_COUNTER_H
#define EIGHTSEGMENTLED_COUNTER_H

#include <Arduino.h>
#include <math.h>
#include <src/debug.h>

class MultiCharacterUpCounter {
public:
    MultiCharacterUpCounter(uint8_t num_digits, uint8_t num_mappings) :
            _num_digits(num_digits),
            _num_mappings(num_mappings) {
        _data = new uint8_t[num_digits];
        //init
        memset(_data, 0, num_digits);
    }

    void increment(){
        debug->trace("increment:_num_digits:" + String(_num_digits));
        debug->trace("increment:_num_mappings:" + String(_num_mappings));
        bool carry = true;
        for(int index = _num_digits - 1; index > -1; index--){
            debug->trace("increment:index:" + String(index));
            if(carry){
                _data[index] = _data[index] + 1;
                debug->trace("increment:data:" + String(_data[index]));
                if(_data[index] >= _num_mappings){
                    _data[index] = 0;
                    carry = true;
                    debug->trace("increment:data:reset:" + String(_data[index]));
                }else{
                    carry = false;
                }
            }
            debug->trace("increment:carry:" + String(carry));
        }
    }

    void data(uint8_t *dest){
        memcpy(dest, _data, _num_digits);
    }
private:
    uint8_t _num_digits;
    uint8_t _num_mappings;
    uint8_t *_data;
};


#endif //EIGHTSEGMENTLED_COUNTER_H
