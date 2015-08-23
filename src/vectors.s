.file "vectors.s"

// Declares weak symbols for exception vectors, so that end-user code
// can override them.
//
// Every handler the user doesn't override will be aliased to "missing_handler".
// The user may also override "missing_handler".  If they do not, a default
// will be supplied (default_missing_handler), which simply hangs.

.include "weak_fn.s.h"

.section .text.reset_handler
    .func reset_handler
    weak_fn reset_handler
    reset_handler:
        push        {r7, lr}
        bl          init_sram
        movs        r0, #0
        movs        r1, #0
        bl          start
        b           abort
    .size reset_handler, . - reset_handler
    .endfunc
    
.section .text.default_missing_handler
    .func default_missing_handler
    .thumb_func
    default_missing_handler:
        b missing_handler
    .size default_missing_handler, . - default_missing_handler
    .endfunc
    
    weak_fn_alias missing_handler,                 default_missing_handler
    
    weak_fn_alias non_maskable_interrupt_handler,  default_missing_handler
    weak_fn_alias hard_fault_handler,              default_missing_handler
    weak_fn_alias mem_manage_handler,              default_missing_handler
    weak_fn_alias bus_fault_handler,               default_missing_handler
    weak_fn_alias usage_fault_handler,             default_missing_handler
    weak_fn_alias sv_call_handler,                 default_missing_handler
    weak_fn_alias debug_monitor_handler,           default_missing_handler
    weak_fn_alias pend_sv_handler,                 default_missing_handler
    weak_fn_alias sys_tick_handler,                default_missing_handler

.section .exception_vectors.thumbv6m
    .globl thumbv6m_exception_table
    .type thumbv6m_exception_table, #object
    .size thumbv6m_exception_table, 0x40
    thumbv6m_exception_table:
        .word __stack
        .word reset_handler
        .word non_maskable_interrupt_handler
        .word hard_fault_handler
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word sv_call_handler
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word pend_sv_handler
        .word sys_tick_handler

.section .exception_vectors.thumbv7m
    .globl thumbv7m_exception_table
    .type thumbv7m_exception_table, #object
    .size thumbv7m_exception_table, 0x40
    thumbv7m_exception_table:
        .word __stack
        .word reset_handler
        .word non_maskable_interrupt_handler
        .word hard_fault_handler
        .word mem_manage_handler
        .word bus_fault_handler
        .word usage_fault_handler
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word 0xFFFFFFFF
        .word sv_call_handler
        .word debug_monitor_handler
        .word 0xFFFFFFFF
        .word pend_sv_handler
        .word sys_tick_handler
