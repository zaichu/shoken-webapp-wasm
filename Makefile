.PHONY: all build run clean

all: clean build run

build:
	trunk build --release

run:
	trunk serve --release

clean:
	rm -rf dist
	cargo clean
