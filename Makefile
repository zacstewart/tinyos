AS = nasm
ASFLAGS = -f elf32
CC = gcc
CFLAGS = -m32 -nostdlib -nostdinc -fno-builtin -fno-stack-protector \
         -nostartfiles -nodefaultlibs -Wall -Wextra -Werror -c
LD = ld
LDFLAGS = -T link.ld -melf_i386
OBJECTS = loader.o kmain.o
QEMU = qemu-system-i386

all: kernel.elf

run: tinyos.iso
	$(QEMU) -cdrom tinyos.iso -monitor stdio

clean:
	rm -rf *.o tinyos.iso kernel.elf

%.o: %.c
	$(CC) $(CFLAGS)  $< -o $@

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

kernel.elf: $(OBJECTS)
	$(LD) $(LDFLAGS) $(OBJECTS) -o kernel.elf

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
