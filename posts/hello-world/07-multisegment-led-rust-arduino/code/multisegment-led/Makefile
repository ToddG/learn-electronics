
.PHONY: help
help:
	# -----------------------------------------------------------------------------
	# Targets:
	#
	#	clean 		: run `cargo clean` for each crate
	#	help 		: show this message
	#	build		: run `cargo build` for each crate
	#	test		: run `cargo test` for the mseg-test crate
	#	libtest		: build `mseg-lib` crate and run `cargo test` for the mseg-test crate
	#	deploy		: use `avrdude` to deploy mseg-bin to device
	#
	# end.
	# -----------------------------------------------------------------------------


.PHONY: clean
clean:
	(cd mseg-lib && cargo clean)
	(cd mseg-bin && cargo clean)
	(cd mseg-test && cargo clean)

.PHONY: build
build:
	(cd mseg-lib && cargo build)
	(cd mseg-bin && cargo build)
	(cd mseg-test && cargo build)

.PHONY: test
test:
	(cd mseg-test && cargo test)

.PHONY: libtest
libtest:
	(cd mseg-lib && cargo build)
	(cd mseg-test && cargo test)

.PHONY: deploy
deploy:
	(cd mseg-bin && avrdude -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:target/avr-atmega328p/debug/mseg-bin.elf:e")
