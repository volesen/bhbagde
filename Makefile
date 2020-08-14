upload: build
	stty -F /dev/ttyACM0 ospeed 1200 | ./tools/bossac -e -w -v -b ./main.bin

build: src/*.rs
	echo "building"
	cargo build
	echo "stripping"
	arm-none-eabi-objcopy -O binary ./target/thumbv6m-none-eabi/debug/metro_m0 ./main.bin

