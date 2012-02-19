all:
	rustc --lib sdl.rc

test:
	rustc --test sdl.rc
	./sdl

clean:
	rm -rf sdl libsdl*.so

