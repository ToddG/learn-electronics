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
	(cd mseg-lib && cargo clean)
	(cd mseg-bin && cargo clean)

.PHONY: build
build:
	(cd mseg-lib && cargo build)
	(cd mseg-bin && cargo build)

.PHONY: deploy
deploy:
	(cd mseg-bin && avrdude -v -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/mseg-bin.elf:e")
