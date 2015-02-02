global _outb
global _inb

_outb:
    mov al, [esp + 8]
    mov dx, [esp + 4]
    out dx, al
    ret

_inb:
    mov dx, [esp + 4]
    in al, dx
    ret
