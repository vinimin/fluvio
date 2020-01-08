all:	build

setup:
	mkdir -p lib

build:	setup
	cargo build --lib
	cp ../../target/debug/libflv_node_client.dylib lib/index.node