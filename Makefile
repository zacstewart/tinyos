AS = nasm
ASFLAGS = -f elf32
LD = i386-elf-ld
LDFLAGS = -T link.ld -melf_i386
QEMU = qemu-system-i386

all: kernel.elf

run: tinyos.iso
	$(QEMU) -cdrom tinyos.iso -monitor stdio

clean:
	rm -rf *.o tinyos.iso kernel.elf

loader.o: loader.s
	nasm -f elf32 loader.s

kernel.elf: loader.o
	$(LD) $(LDFLAGS) loader.o -o kernel.elf

tinyos.iso: kernel.elf
	cp kernel.elf iso/boot/kernel.elf
	mkisofs -R                              \
                -b boot/grub/stage2_eltorito    \
                -no-emul-boot                   \
                -boot-load-size 4               \
                -A os                           \
                -input-charset utf8             \
                -quiet                          \
                -boot-info-table                \
                -o tinyos.iso                   \
                iso
