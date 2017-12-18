all: build

build-release:
	docker run -it --rm -v $$(pwd):/home/rust/src omnijar/rust cargo build --release

build:
	cargo build

run-release:
	cargo run --release

run:
	cargo run

test:
	cargo test

clean:
	rm -rf target/
