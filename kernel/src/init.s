    .section .bss
    .align 4096
stack_bottom:
    .skip 4096 * 4
stack_top:

    .section .text
    .global _start

# Initialise the kernel stack and call kernel_main
#
# Arguments:
# rdi - Pointer to the BootInfo struct. This is just passed onto kernel_main
_start:
    movabs $stack_top, %rsp
    call kernel_main
