release: main.uf2
	echo "uploading"
	cp main.uf2 /media/volesen/BHBADGEBOOT/

main.uf2: main.bin
	echo "packing"
	uf2conv main.bin --base 0x2000 --output main.uf2

main.bin: src/*.rs
	echo "building"
	cargo objcopy --features unproven --release -- -O binary main.bin

clean:
	rm main.bin main.uf2
