#![no_std]

use core::option::Option;
use core::option::Option::{Some, None};

/// per the ARMv7-M Architecture Reference Manual
/// Section B1.5.2 (Table B1-4)
/// 
/// Applies to all Cortex-M3, -M3 and -M7 microcontrollers.

#[no_mangle] #[link_section=".exception_vectors.thumbv7m"]
#[allow(non_upper_case_globals)]
pub static thumbv7m_exception_table : [Option<unsafe extern fn()>; 16] = [
    Some(__stack),
    Some(reset_handler),
    Some(non_maskable_interrupt_handler),
    Some(hard_fault_handler),
    Some(mem_manage_handler),
    Some(bus_fault_handler),
    Some(usage_fault_handler),
    None,
    None,
    None,
    None,
    Some(sv_call_handler),
    Some(debug_monitor_handler),
    None,
    Some(pend_sv_handler),
    Some(sys_tick_handler),
];

#[no_mangle] #[link_section=".exception_vectors.thumbv6m"]
#[allow(non_upper_case_globals)]
pub static thumbv6m_exception_table : [Option<unsafe extern fn()>; 16] = [
    Some(__stack),
    Some(reset_handler),
    Some(non_maskable_interrupt_handler),
    Some(hard_fault_handler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(sv_call_handler),
    None,
    None,
    Some(pend_sv_handler),
    Some(sys_tick_handler),
];

extern {
    fn __stack();
    fn reset_handler();
    fn non_maskable_interrupt_handler();
    fn hard_fault_handler();
    fn mem_manage_handler();
    fn bus_fault_handler();
    fn usage_fault_handler();
    fn sv_call_handler();
    fn debug_monitor_handler();
    fn pend_sv_handler();
    fn sys_tick_handler();
}

