.PHONY: all build copy clean

all: build copy

build:
	trunk build --release

copy:
	mkdir -p docs
	cp -r dist/* docs/

clean:
	rm -rf dist docs
