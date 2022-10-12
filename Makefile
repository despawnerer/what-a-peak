ARCH = x86_64-unknown-linux-gnu

.PHONY: build
build:
	cargo lambda build --release
	rm -rf ./build
	mkdir -p ./build
	cp -v ./target/lambda/what-a-peak/bootstrap ./build/bootstrap
