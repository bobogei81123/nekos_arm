/// Call the function provided by parameter `\handler` after saving the exception context. Provide
/// the context as the first parameter to '\handler'.
.macro CALL_WITH_CONTEXT handler
    // Make room on the stack for the exception context.
    sub    sp,  sp,  #16 * 17

    // Store all general purpose registers on the stack.
    stp    x0,  x1,  [sp, #16 * 0]
    stp    x2,  x3,  [sp, #16 * 1]
    stp    x4,  x5,  [sp, #16 * 2]
    stp    x6,  x7,  [sp, #16 * 3]
    stp    x8,  x9,  [sp, #16 * 4]
    stp    x10, x11, [sp, #16 * 5]
    stp    x12, x13, [sp, #16 * 6]
    stp    x14, x15, [sp, #16 * 7]
    stp    x16, x17, [sp, #16 * 8]
    stp    x18, x19, [sp, #16 * 9]
    stp    x20, x21, [sp, #16 * 10]
    stp    x22, x23, [sp, #16 * 11]
    stp    x24, x25, [sp, #16 * 12]
    stp    x26, x27, [sp, #16 * 13]
    stp    x28, x29, [sp, #16 * 14]

    // Add the exception link register (ELR_EL1) and the saved program status (SPSR_EL1).
    mrs    x1,  ELR_EL1
    mrs    x2,  SPSR_EL1

    stp    lr,  x1,  [sp, #16 * 15]
    str    x2,       [sp, #16 * 16]

    // x0 is the first argument for the function called through `\handler`.
    mov    x0,  sp

    // Call `\handler`.
    bl     \handler

    // After returning from exception handling code, replay the saved context and return via `eret`.
    b      __exception_restore_context
.endm

//--------------------------------------------------------------------------------------------------
// The exception vector table.
//--------------------------------------------------------------------------------------------------
.section .exception_vectors, "ax", @progbits

// Align by 2^11 bytes, as demanded by ARMv8-A. Same as ALIGN(2048) in an ld script.
.align 11

// Export a symbol for the Rust code to use.
__exception_vector_start:

.org 0x200
    CALL_WITH_CONTEXT current_elx_synchronous

    .section .text

__exception_restore_context:
    ldr    w19,      [sp, #16 * 16]
    ldp    lr,  x20, [sp, #16 * 15]

    msr    SPSR_EL1, x19
    msr    ELR_EL1,  x20

    ldp    x0,  x1,  [sp, #16 * 0]
    ldp    x2,  x3,  [sp, #16 * 1]
    ldp    x4,  x5,  [sp, #16 * 2]
    ldp    x6,  x7,  [sp, #16 * 3]
    ldp    x8,  x9,  [sp, #16 * 4]
    ldp    x10, x11, [sp, #16 * 5]
    ldp    x12, x13, [sp, #16 * 6]
    ldp    x14, x15, [sp, #16 * 7]
    ldp    x16, x17, [sp, #16 * 8]
    ldp    x18, x19, [sp, #16 * 9]
    ldp    x20, x21, [sp, #16 * 10]
    ldp    x22, x23, [sp, #16 * 11]
    ldp    x24, x25, [sp, #16 * 12]
    ldp    x26, x27, [sp, #16 * 13]
    ldp    x28, x29, [sp, #16 * 14]

    add    sp,  sp,  #16 * 17

    eret
