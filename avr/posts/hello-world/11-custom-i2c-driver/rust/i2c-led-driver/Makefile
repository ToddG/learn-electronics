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
	(cd i2c-led-driver-bin && cargo clean)

.PHONY: build
build:
	(cd i2c-led-driver-bin && cargo build)

.PHONY: deploy
deploy:
	(cd i2c-led-driver-bin && avrdude -v -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/i2c-led-driver-bin.elf:e")
