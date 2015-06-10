AS = nasm
ASFLAGS = -f elf32
RUSTC = rustc
RUSTFLAGS = -L $(HOME)/.multirust/toolchains/my-rust/lib/rustlib/i686-unknown-linux-gnu/lib/ --target i686-unknown-linux-gnu -O --crate-type lib --emit obj
LD = ld
LDFLAGS = -T link.ld -melf_i386
OBJECTS = loader.o io.o kmain.o
QEMU = qemu-system-i386

all: kernel.elf

run: tinyos.iso
	$(QEMU) -cdrom tinyos.iso -monitor stdio -serial file:"log.txt"

clean:
	rm -rf *.o tinyos.iso kernel.elf

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

%.o: %.rs
	$(RUSTC) $(RUSTFLAGS) -o $@ $<

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
