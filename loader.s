global loader

MAGIC_NUMBER equ 0x1BADB002
CHECKSUM equ -MAGIC_NUMBER

section .text:
align 4
    dd MAGIC_NUMBER
    dd CHECKSUM

loader:
    mov eax, 0xCAFEBABE
.loop:
  jmp .loop
