# 04 Characters

## Goal

Create a library to parse an input and display one of 2 characters:

    HIGH -> display "A"
    LOW  -> display "B"

## Build

Same as 03.

## Preliminaries

Now it's time to start looking at the language spec:

    https://www.arduino.cc/reference/en/

Ok, I have a ton of questions about the language now, passing by reference vs. by value,
as well as what is the proper design pattern here? Is this C? C++? WTF? And here I stumble
upon this post:

    https://paulmurraycbr.github.io/ArduinoTheOOWay.html

And I think I am in love, because he is proposing a sane way to use OO in the exact way
that my all time fave, Yegor, does here:

    https://www.yegor256.com/elegant-objects.html

And now that we are starting to write real code, it's time to switch to a
*real* editor, one that supports refactoring, code completion, etc. _CLION_ is
my choice (I have a personal license for the entire jetbrains tool suite):

* https://plugins.jetbrains.com/clion
* https://plugins.jetbrains.com/plugin/7889-arduino

## Sketch


