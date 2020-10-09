#!/usr/bin/env python3

# NOTES
# 
# See:
#   * https://www.instructables.com/id/Arduino-Timer-Interrupts/
#       """compare match register = [ 16,000,000Hz/ (prescaler * desired interrupt frequency) ] - 1"""
#   * http://www.8bit-era.cz/arduino-timer-interrupts-calculator.html
#   * https://oscarliang.com/arduino-timer-and-interrupt-tutorial/
#   * https://circuitdigest.com/microcontroller-projects/arduino-timer-tutorial
#   * https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-hc-sr04.rs


AVR_UNO_PRESCALERS = [ 1, 8, 64, 256, 1024]
WANTED_ISR_FREQUENCIES = [1e-12, 1e-9, 1e-6, 1e-3, 1e0, 1e1, 1e3, 1e6, 1e9, 1e12, 1e16, 1e32, 1e64, 1e128]


def compare_match_register_values(freqs):
    for f in freqs:
        for p in AVR_UNO_PRESCALERS:
            cmr = 16e6 / (p * f) -1
            yield (f, p, cmr, timers(cmr))


def timers(cmr):
    # 2**8 = 256
    if cmr < 2**8: 
        return [0, 2]
    # 2**16 = 65536
    if cmr < 2**16:
        return [1]

def dump(items):
    print("# ------------------------------------")
    print("# interrupt frequency: f")
    print("# prescaler: p")
    print("# compare match register value: cmr")
    print("# timers: t")
    print("# ------------------------------------")
    for (f, p, cmr, t) in items:
        print(f'"f: {f}, p: {p}, cmr: {cmr}, t: {t}"')

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        wanted_freqs = list(map(lambda x: int(x), sys.argv[1:]))
        dump(compare_match_register_values(wanted_freqs))
    else:
        dump(compare_match_register_values(WANTED_ISR_FREQUENCIES))
