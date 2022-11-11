aarch64-elf-as -c boot.s -o build/boot.o
aarch64-elf-gcc -ffreestanding -c kernel/kernel.c -o build/kernel.o -O2 -Wall -Wextra
aarch64-elf-gcc -T linker.ld -o build/test.elf -ffreestanding -O2 -nostdlib build/boot.o build/kernel.o -lgcc 
aarch64-elf-objcopy build/test.elf -O binary bin/kernel8.img
qemu-system-aarch64 -M raspi3b -serial stdio -kernel bin/kernel8.img
