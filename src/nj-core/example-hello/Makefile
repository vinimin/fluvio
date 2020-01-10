
DYLIB = ../../../target/debug/libnj_sys_example_hello.dylib

all:	build

setup:
	mkdir -p dylib

build:	setup
	cargo build --lib
	cp $(DYLIB) dylib/index.node

test:
	node test.js


clean:
	rm -rf dylib

