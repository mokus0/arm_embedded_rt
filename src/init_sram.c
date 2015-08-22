#include <stdint.h>

// these symbols are defined in the linker script, and describe the location
// of various sections of memory.  They are declared as concrete types (u32)
// but their contents are meaningless (and completely unsafe to touch) - it
// is their addresses we need.
//
// we indend to copy/zero these sections one full word at a time, so these
// must all be word-aligned by the linker script.

/// The first word in the .data section in SRAM
extern uint32_t __data_start_sram;

/// The first word in the .data section in FLASH
extern uint32_t __data_start_flash;

/// The first word *after* the .data section in FLASH
extern uint32_t __data_end_flash;

/// The first word in the .bss section in SRAM
extern uint32_t __bss_start;

/// The first word *after* the .bss section in SRAM
extern uint32_t __bss_end;

/// Load .data section from FLASH to SRAM and initialize .bss section
__attribute__((weak))
void init_sram(void)
{
    // Let's get this party started!
    
    // Copy .data section from FLASH to SRAM
    {
        const uint32_t *const end = &__data_end_flash;
        
        uint32_t * src = &__data_start_flash;
        uint32_t * dst = &__data_start_sram;
        
        while (src < end)
        {
            *dst++ = *src++;
        }
    }

    // Initialize .bss section in SRAM
    {
        const uint32_t *const end = &__bss_end;
        uint32_t * dst = &__bss_start;
        
        while (dst < end)
        {
            *dst++ = 0;
        }
    }
}