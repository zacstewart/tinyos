global loader                 ; the entry symbol for ELF

ALI    equ 1 << 0           ; align loaded modules on page boundaries
MEMINFO  equ 1 << 1           ; provide memory map
FLAGS    equ ALI | MEMINFO  ; the multiboot flags field
MAGIC    equ 0x1BADB002       ; define the magic number constant
CHECKSUM equ -(MAGIC + FLAGS) ; calculate the checksum (magic number + flags + checksum should equal 0)

section .text:                ; start of the text (code) section
align 4                       ; the code must be 4 byte aligned
    dd MAGIC                  ; write the magic number to the machine code
    dd FLAGS
    dd CHECKSUM               ; and the checksum

loader:                       ; the loader label (defined as entry point in linker script)
    mov eax, 0xCAFEBABE       ; place the number 0xCAFEBABE in the register eax
.loop:
    jmp .loop                 ; loop forever
