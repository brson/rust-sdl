RUSTC ?= rustc
RUSTFLAGS ?=

RUST_SRC = $(shell find . -type f -name '*.rs')

all:
	rustc sdl.rc
	touch libsdl.dummy

test:
	rustc --test sdl.rc
	./sdl

clean:
	rm -rf sdl libsdl*.so

