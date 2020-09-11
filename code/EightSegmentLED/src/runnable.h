//
// Created by todd on 9/11/20.
//

#ifndef EIGHTSEGMENTLED_RUNNABLE_H
#define EIGHTSEGMENTLED_RUNNABLE_H

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

#endif //EIGHTSEGMENTLED_RUNNABLE_H
