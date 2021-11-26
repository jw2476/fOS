cargo run kernel.bf -o tmp/kernel.o
i686-elf-as boot.s -o tmp/boot.o
i686-elf-gcc -T linker.ld -o fOS.bin -ffreestanding -O2 -nostdlib tmp/boot.o tmp/kernel.o -lgcc
qemu-system-i386 --kernel fOS.bin