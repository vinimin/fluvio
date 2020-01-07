bindgen:
	cargo install bindgen

header = vendor/nodejs_13/node_api.h

generate: bindgen
	bindgen $(header) -o src/binding.rs

