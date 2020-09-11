//
// Created by todd on 9/11/20.
//

#ifndef EIGHTSEGMENTLED_LED_H
#define EIGHTSEGMENTLED_LED_H

#include <Arduino.h>
#include "runnable.h"
#include "charactermap.h"

/*
 * SM4105W6 Eight Segment LED
 * https://www.velleman.eu/downloads/29/infosheets/vmp502_sma42056etc.pdf
 *
 *  Pins
 *      A    B   C   D   E   F   G   DP
 *      7    6   4   2   1   9   10  5
 *
 */
class EightSegmentLEDCommonAnode : public Runnable {
    // digital IO pins
    byte p7a;
    byte p6b;
    byte p4c;
    byte p2d;
    byte p1e;
    byte p9f;
    byte p10g;
    byte p5dp;
public:
    // character map converts inputs to segment outputs
    CharacterMap *characterMap;
    int input = 0;

    EightSegmentLEDCommonAnode(
            CharacterMap *characterMap,
            byte ledPin01E_digitialIOPin,
            byte ledPin02D_digitialIOPin,
            byte ledPin04C_digitialIOPin,
            byte ledPin05DP_digitialIOPin,
            byte ledPin06B_digitialIOPin,
            byte ledPin07A_digitialIOPin,
            byte ledPin09F_digitialIOPin,
            byte ledPin10G_digitialIOPin
    )
            :
            characterMap(characterMap),
            p7a(ledPin07A_digitialIOPin),
            p6b(ledPin06B_digitialIOPin),
            p4c(ledPin04C_digitialIOPin),
            p2d(ledPin02D_digitialIOPin),
            p1e(ledPin01E_digitialIOPin),
            p9f(ledPin09F_digitialIOPin),
            p10g(ledPin10G_digitialIOPin),
            p5dp(ledPin05DP_digitialIOPin) {};

    void setup() {
        pinMode(p7a, OUTPUT);
        pinMode(p6b, OUTPUT);
        pinMode(p4c, OUTPUT);
        pinMode(p2d, OUTPUT);
        pinMode(p1e, OUTPUT);
        pinMode(p9f, OUTPUT);
        pinMode(p10g, OUTPUT);
        pinMode(p5dp, OUTPUT);
        digitalWrite(p7a, HIGH);
        digitalWrite(p6b, HIGH);
        digitalWrite(p4c, HIGH);
        digitalWrite(p2d, HIGH);
        digitalWrite(p1e, HIGH);
        digitalWrite(p9f, HIGH);
        digitalWrite(p10g, HIGH);
        digitalWrite(p5dp, HIGH);
    }

    void loop() {
        /*
         * 1. read the input
         * 2. use a character map to generate display segments
         * 3. map display segments to digital io OUTPUT pins
         */
        const bool *output = characterMap->display(input);
        digitalWrite(p7a, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_A]);
        digitalWrite(p6b, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_B]);
        digitalWrite(p4c, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_C]);
        digitalWrite(p2d, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_D]);
        digitalWrite(p1e, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_E]);
        digitalWrite(p9f, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_F]);
        digitalWrite(p10g, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_G]);
        digitalWrite(p5dp, !output[GenericEightSegmentLEDCharacterMap::SEGMENT_INDEX_DP]);
    }
};


#endif //EIGHTSEGMENTLED_LED_H
