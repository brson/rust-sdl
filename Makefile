.PHONY: build clean demo

build:
	rustc src/sdl/lib.rs

clean:
	rm *.rlib
	rm demo

demo:
	rustc -o demo -L. src/sdl-demo/main.rs
