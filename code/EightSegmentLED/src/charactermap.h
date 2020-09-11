//
// Created by todd on 9/11/20.
//
#ifndef EIGHTSEGMENTLED_CHARACTERMAP_H
#define EIGHTSEGMENTLED_CHARACTERMAP_H

#include <Arduino.h>

/**
 * Abstract base class for CharacterMap
 */
class CharacterMap {
public:
    /**
     *
     * @param input NUM_ZERO through NUM_NINE or CHAR_A through CHAR_Z.
     * @return pointer to bool array representing LED segments: A,B,C,D,E,F,G,DP.
     *
     */
    virtual const bool *display(uint8_t input) = 0;

    /**
     * Number of display elements
     */
     virtual const uint8_t num_segments() = 0;

    /**
     * Number of input mappings
     */
    virtual const uint8_t num_mappings() = 0;
};

/*
 * Generic 8 Segment LED
 *
 *  +--------------------------------------------------------------+
 * |                                                              |
 * |                                                              |
 * |                        A                                     |
 * |            |----------------------------|                    |
 * |            |----------------------------|                    |
 * |            +--+                      +--+                    |
 * |            |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |            |  |                      |  | B                  |
 * |      F     |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |            +--+          G           +--+                    |
 * |            |----------------------------|                    |
 * |            |----------------------------|                    |
 * |            +--+                      +--+                    |
 * |            |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |       E    |  |                      |  | C                  |
 * |            |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |            |  |                      |  |                    |
 * |            +--+                      +--+                    |
 * |            |----------------------------|      +-+           |
 * |            |----------------------------|      +-+   DP      |
 * |                        D                                     |
 * |                                                              |
 * +--------------------------------------------------------------+
 *
 */
class GenericEightSegmentLEDCharacterMap : public CharacterMap {
    static const uint8_t numSegments = 8;
    static const uint8_t numMappings = 37;
public:
    // segment indexes
    static const uint8_t SEGMENT_INDEX_A = 0;
    static const uint8_t SEGMENT_INDEX_B = 1;
    static const uint8_t SEGMENT_INDEX_C = 2;
    static const uint8_t SEGMENT_INDEX_D = 3;
    static const uint8_t SEGMENT_INDEX_E = 4;
    static const uint8_t SEGMENT_INDEX_F = 5;
    static const uint8_t SEGMENT_INDEX_G = 6;
    static const uint8_t SEGMENT_INDEX_DP = 7;

    // numbers
    static const uint8_t NUM_ZERO = 0;
    static const uint8_t NUM_ONE = 1;
    static const uint8_t NUM_TWO = 2;
    static const uint8_t NUM_THREE = 3;
    static const uint8_t NUM_FOUR = 4;
    static const uint8_t NUM_FIVE = 5;
    static const uint8_t NUM_SIX = 6;
    static const uint8_t NUM_SEVEN = 7;
    static const uint8_t NUM_EIGHT = 8;
    static const uint8_t NUM_NINE = 9;

    // characters
    static const uint8_t CHAR_A = 11;
    static const uint8_t CHAR_B = 12;
    static const uint8_t CHAR_C = 13;
    static const uint8_t CHAR_D = 14;
    static const uint8_t CHAR_E = 15;
    static const uint8_t CHAR_F = 16;
    static const uint8_t CHAR_G = 17;
    static const uint8_t CHAR_H = 18;
    static const uint8_t CHAR_I = 19;
    static const uint8_t CHAR_J = 20;
    static const uint8_t CHAR_K = 21;
    static const uint8_t CHAR_L = 22;
    static const uint8_t CHAR_M = 23;
    static const uint8_t CHAR_N = 24;
    static const uint8_t CHAR_O = 25;
    static const uint8_t CHAR_P = 26;
    static const uint8_t CHAR_Q = 27;
    static const uint8_t CHAR_R = 28;
    static const uint8_t CHAR_S = 29;
    static const uint8_t CHAR_T = 30;
    static const uint8_t CHAR_U = 31;
    static const uint8_t CHAR_V = 32;
    static const uint8_t CHAR_W = 33;
    static const uint8_t CHAR_X = 34;
    static const uint8_t CHAR_Y = 35;
    static const uint8_t CHAR_Z = 36;

    const bool *display(uint8_t input) {
        if (input < numMappings) {
            return displayArr[input];
        } else {
            return displayArr[numMappings - 1];
        }
    }

    const uint8_t num_segments(){
        return numSegments;
    }

    const uint8_t num_mappings(){
        return numMappings;
    }



    static GenericEightSegmentLEDCharacterMap *characterMap;
private:
    const bool displayArr[numMappings][numSegments] = {
            // led segments (see SEGMENT_INDEX_XX)
            // A      B        C       D    E      F       G    DP
            // NUM_ZERO
            {true,  true,  true,  true,  true,  true,  false, false},
            // NUM_ONE
            {false, true,  true,  false, false, false, false, false},
            // NUM_TWO
            {true,  true,  false, true,  true,  false, true,  false},
            // NUM_THREE
            {true,  true,  true,  true,  false, false, true,  false},
            // NUM_FOUR
            {false, true,  true,  false, false, true,  true,  false},
            // NUM_FIVE
            {true,  false, true,  true,  false, true,  true,  false},
            // NUM_SIX
            {true,  false, true,  true,  true,  true,  true,  false},
            // NUM_SEVEN
            {true,  true,  true,  false, false, false, false, false},
            // NUM_EIGHT
            {true,  true,  true,  true,  true,  true,  true,  false},
            // NUM_NINE
            {true,  true,  true,  false, false, true,  true,  false},
            // CHAR_A
            {true,  true,  true,  false, true,  true,  true,  true},
            // CHAR_B
            {true,  true,  true,  true,  true,  true,  true,  true},
            // CHAR_C
            {true,  false, false, true,  true,  true,  false, true},
            // CHAR_D
            {true,  true,  true,  true,  true,  true,  false, true},
            // CHAR_E
            {true,  false, false, true,  true,  true,  true,  true},
            // CHAR_F
            {true,  false, false, false, true,  true,  true,  true},
            // CHAR_G
            {true,  false, true,  true,  true,  true,  true,  true},
            // CHAR_H
            {false, true,  true,  false, true,  true,  true,  true},
            // CHAR_I
            {true,  true,  true,  true,  false, false, false, true},
            // CHAR_J
            {false, true,  true,  true,  true,  false, false, true},
            // CHAR_K
            {false, false, true,  false, true,  true,  false, true},
            // CHAR_L
            {false, false, false, true,  true,  true,  false, true},
            // CHAR_M
            {true,  true,  true,  false, true,  true,  false, true},
            // CHAR_N
            {false, false, true,  false, true,  false, true,  true},
            // CHAR_O
            {true,  true,  true,  true,  true,  true,  false, true},
            // CHAR_P
            {true,  true,  false, false, true,  true,  true,  true},
            // CHAR_Q
            {true,  false, false, true,  false, false, true,  true},
            // CHAR_R
            {true,  true,  true,  false, true,  true,  true,  true},
            // CHAR_S
            {true,  false, true,  true,  false, true,  true,  true},
            // CHAR_T
            {true,  true,  true,  true,  false, false, false, true},
            // CHAR_U
            {false, true,  true,  true,  true,  true,  false, true},
            // CHAR_V
            {false, false, true,  true,  true,  false, false, true},
            // CHAR_W
            {true,  false, false, true,  false, false, true,  true},
            // CHAR_X
            {false, true,  true,  false, true,  true,  false, true},
            // CHAR_Y
            {false, true,  false, false, true,  true,  true,  true},
            // CHAR_Z
            {true,  true,  false, true,  true,  false, true,  true},
            // ERROR
            {false, true,  true,  false, false, true,  true,  true}
    };
};

#endif //EIGHTSEGMENTLED_CHARACTERMAP_H
