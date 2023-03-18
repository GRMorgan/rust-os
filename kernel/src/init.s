    .section .bss
    .align 4096
stack_bottom:
    .skip 4096 * 4
stack_top:

    .section .text
    .global _start
_start:
    movabs $stack_top, %rsp
    call kernel_main
