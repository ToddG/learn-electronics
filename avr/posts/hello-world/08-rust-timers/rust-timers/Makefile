.PHONY: help
help:
	# -----------------------------------------------------------------------------
	# Targets:
	#
	#	build		: run `cargo build` for each crate
	#	clean 		: run `cargo clean` for each crate
	#	deploy		: use `avrdude` to deploy mseg-bin to device
	#	help 		: show this message
	#
	# end.
	# -----------------------------------------------------------------------------


.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build

.PHONY: deploy
deploy:
	avrdude -v -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/rust-timers.elf:e"
