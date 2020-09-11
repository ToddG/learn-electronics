#include <Arduino.h>

// Using Runnable pattern from https://paulmurraycbr.github.io/ArduinoTheOOWay.html
class Runnable {
    static Runnable *headRunnable;
    Runnable *nextRunnable;

public:
    Runnable() {
        nextRunnable = headRunnable;
        headRunnable = this;
    }

    virtual void setup() = 0;

    virtual void loop() = 0;

    static void setupAll() {
        for (Runnable *r = headRunnable; r; r = r->nextRunnable)
            r->setup();
    }

    static void loopAll() {
        for (Runnable *r = headRunnable; r; r = r->nextRunnable)
            r->loop();
    }

};

Runnable *Runnable::headRunnable = NULL;

class CharacterMap {
public:
    /**
     *
     * @param input NUM_ZERO through NUM_NINE or CHAR_A through CHAR_Z.
     * @return pointer to bool array representing LED segments: A,B,C,D,E,F,G,DP.
     *
     */
    virtual const bool *display(uint8_t input) = 0;
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
    static const int numSegments = 8;
public:
    static const int numMappings = 37;
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

private:
    const bool displayArr[numMappings][numSegments] = {
            // led segments (see SEGMENT_INDEX_XX)
            // A      B        C       D    E      F       G    DP
            // NUM_ZERO
            {true,  true, true,  true,  true,  true,  false, false},
            // NUM_ONE
            {false, true, true,  false, false, false, false, false},
            // NUM_TWO
            {true,  true, false, true,  true,  false, true,  false},
            // NUM_THREE
            {true,  true, true, true,  false,  false, true,  false},
            // NUM_FOUR
            {false, true, true, false, false, true, true, false},
            // NUM_FIVE
            {true, false, true, true,  false,  true, true,  false},
            // NUM_SIX
            {true, false, true, true,  true,  true, true,  false},
            // NUM_SEVEN
            {true, true, true, false,  false,  false, false,  false},
            // NUM_EIGHT
            {true, true, true, true,  true,  true, true,  false},
            // NUM_NINE
            {true, true, true, false,  false,  true, true,  false},
            // CHAR_A
            {true, true, true, false,  true,  true, true,  true},
            // CHAR_B
            {true, true, true, true,  true,  true, true,  true},
            // CHAR_C
            {true, false, false, true,  true,  true, false,  true},
            // CHAR_D
            {true, true, true, true,  true,  true, false,  true},
            // CHAR_E
            {true, false, false, true,  true,  true, true,  true},
            // CHAR_F
            {true, false, false, false,  true,  true, true,  true},
            // CHAR_G
            {true, false, true, true,  true,  true, true,  true},
            // CHAR_H
            {false, true, true, false,  true,  true, true,  true},
            // CHAR_I
            {true, true, true, true,  false,  false, false,  true},
            // CHAR_J
            {false, true, true, true,  true,  false, false,  true},
            // CHAR_K
            {false, false, true, false,  true,  true, false,  true},
            // CHAR_L
            {false, false, false, true,  true,  true, false,  true},
            // CHAR_M
            {true, true, true, false,  true,  true, false,  true},
            // CHAR_N
            {false, false, true, false,  true,  false, true,  true},
            // CHAR_O
            {true, true, true, true,  true,  true, false,  true},
            // CHAR_P
            {true, true, false, false,  true,  true, true,  true},
            // CHAR_Q
            {true, false, false, true,  false,  false, true,  true},
            // CHAR_R
            {true, true, true, false,  true,  true, true,  true},
            // CHAR_S
            {true, false, true, true,  false,  true, true,  true},
            // CHAR_T
            {true, true, true, true,  false,  false, false,  true},
            // CHAR_U
            {false, true, true, true,  true,  true, false,  true},
            // CHAR_V
            {false, false, true, true,  true,  false, false,  true},
            // CHAR_W
            {true, false, false, true,  false,  false, true,  true},
            // CHAR_X
            {false, true, true, false,  true,  true, false,  true},
            // CHAR_Y
            {false, true, false, false,  true,  true, true,  true},
            // CHAR_Z
            {true, true, false, true,  true,  false, true,  true},
            // ERROR
            {false,  true, true,  false,  false,  true,  true,  true}
    };
};

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
    // character map converts inputs to segment outputs
    CharacterMap *characterMap;
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

class InputGenerator : Runnable {
    int maxCharacter;
    int sleep;
    EightSegmentLEDCommonAnode *led;

public:
    InputGenerator(int maxCharacter, int sleep, EightSegmentLEDCommonAnode *led)
            : maxCharacter(maxCharacter), sleep(sleep), led(led) {}

private:
    void setup() {
        Serial.begin(115200);
        Serial.print("\nInput generator: \nmaxCharacter = ");
        Serial.println(maxCharacter);
        Serial.println("\nsleep = ");
        Serial.println(sleep);
    }

    void loop() {
        Serial.print("\nled [pre]: ");
        Serial.println(led->input);
        led->input = (led->input + 1) % maxCharacter;
        Serial.print("\nled [post]: ");
        Serial.println(led->input);
        delay(sleep);
    }
};

GenericEightSegmentLEDCharacterMap cm = GenericEightSegmentLEDCharacterMap();
EightSegmentLEDCommonAnode led = EightSegmentLEDCommonAnode(
        &cm, 2, 3, 4, 5,
        6, 7, 8, 9);
InputGenerator gen = InputGenerator(GenericEightSegmentLEDCharacterMap::numMappings, 500, &led);

void setup() {
    Runnable::setupAll();
}

void loop() {
    Runnable::loopAll();
}
