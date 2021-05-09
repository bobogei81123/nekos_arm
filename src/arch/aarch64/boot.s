.globl _start
.extern __EXT_FDT_PTR
.extern __EXT_STACK_END

.section ".text.boot"

_start:
    ldr     x30, =__EXT_FDT_PTR
    str     x0, [x30]
    ldr     x30, =__EXT_STACK_END
    mov     sp, x30
    bl      main

// TODO: get PSCI address from FDT instead
.equ PSCI_SYSTEM_OFF, 0x84000008
.globl __system_off
__system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0

// vim: filetype=arm
