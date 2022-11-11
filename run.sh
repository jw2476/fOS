cd kernel && cpp kernel.bf > ../build/linked.bf && cd ..
cargo run build/linked.bf -o build/kernel.o
aarch64-elf-as boot.s -o build/boot.o
aarch64-elf-gcc -T linker.ld -o bin/fOS.bin -ffreestanding -O2 -nostdlib build/boot.o build/kernel.o -lgcc
qemu-system-aarch64 -kernel bin/fOS.bin -machine raspi3b -serial stdio