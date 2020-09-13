//
// Created by todd on 9/12/20.
//

#ifndef EIGHTSEGMENTLED_TESTS_H
#define EIGHTSEGMENTLED_TESTS_H

#include <Arduino.h>
#include <src/debug.h>
#include <src/counter.h>

void test_eq(uint8_t expected, uint8_t actual) {
    if (expected != actual) {
        debug->error("********" + String(expected) + " ne " + String(actual) + "********");
    } else {
        debug->debug("OK: " + String(expected) + " eq " + String(actual));
    }
}

void test_neq(uint8_t expected, uint8_t actual) {
    if (expected == actual) {
        debug->error("********" + String(expected) + " eq " + String(actual) + "********");
    } else {
        debug->debug("OK: " + String(expected) + " neq " + String(actual));
    }
}


void run_tests() {
    Serial.println("----------------------------");
    Serial.println("START TESTS");

    Serial.println("Test: test_eq");
    test_eq(0, 0);
    test_eq(1, 1);

    Serial.println("Test: test_neq");
    test_neq(1, 0);
    test_neq(0, 1);

    Serial.println("Test: upcounter one digit");
    MultiCharacterUpCounter *c1 = new MultiCharacterUpCounter(1, 2);
    uint8_t *buf = new uint8_t[1];
    memset(buf, 0, 1);
    test_eq(0, buf[0]); // expect 0 in buffer, as that's what we init
    c1->data(buf);
    test_eq(0, buf[0]); // increment not yet called, expect zero

    // first increment -> 1
    c1->increment();
    c1->data(buf);
    test_eq(1, buf[0]); // increment called once, expect one
    c1->increment();
    c1->data(buf);
    test_eq(0, buf[0]); // increment called twice modulo 2, expect zero
    c1->increment();
    c1->data(buf);
    test_eq(1, buf[0]); // increment called 3x modulo 2, expect one
    c1->increment();
    c1->data(buf);
    test_eq(0, buf[0]); // increment called 4x modulo 2, expect zero

    Serial.println("Test: upcounter two digits");
    MultiCharacterUpCounter *c2 = new MultiCharacterUpCounter(2, 2);
    uint8_t *buf2 = new uint8_t[2];
    memset(buf2, 0, 2);
    test_eq(0, buf2[0]);
    test_eq(0, buf2[1]);
    c2->increment(); // 01
    c2->data(buf2);
    test_eq(0, buf2[0]);
    test_eq(1, buf2[1]);
    c2->increment(); // 10
    c2->data(buf2);
    test_eq(1, buf2[0]);
    test_eq(0, buf2[1]);
    c2->increment(); // 11
    c2->data(buf2);
    test_eq(1, buf2[0]);
    test_eq(1, buf2[1]);
    c2->increment(); // 11
    c2->data(buf2);
    test_eq(0, buf2[0]);
    test_eq(0, buf2[1]);

    Serial.println("END TESTS");
    Serial.println("----------------------------");
    delete c1;
    delete buf;
    delete c2;
    delete buf2;
}


#endif //EIGHTSEGMENTLED_TESTS_H
