.global  _start
.section .text
_start:
    mov $0, %rax
    syscall
    mov $14, %rax
    mov $message_start, %rdi
    mov $message_start, %rsi
    mov $message_end, %rdx
    sub %rsi, %rdx
    syscall
    mov $0, %rax
    syscall
.section .rodata
message_start:
    .ascii  "Hello, world\n"
message_end:
