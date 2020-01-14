
DYLIB = ../../../target/debug/libnj_example_hello_promise.dylib

all:	build

setup:
	mkdir -p dylib

build:	setup
	cargo build --lib
	cp $(DYLIB) dylib/index.node

test:	build
	node test.js


clean:
	rm -rf dylib

