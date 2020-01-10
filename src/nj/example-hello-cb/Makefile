
DYLIB = ../../../target/debug/libnj_sys_example_hello_cb.dylib

all:	build

setup:
	mkdir -p dylib

build:	setup
	cargo build --lib
	cp $(DYLIB) dylib/index.node

test:	build
	node test_cb.js


clean:
	rm -rf dylib

