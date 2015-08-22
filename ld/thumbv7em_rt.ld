ENTRY(thumbv7m_exception_table)

SECTIONS {
    .vect.exc VECTOR_BASE : {
        FILL(0xFF)
        KEEP(*(.exception_vectors.thumbv7m))
    } > FLASH
    
    .vect.nvic NVIC_BASE : {
        FILL(0xFF)
        KEEP(*(.nvic))
    } > FLASH
    
    /DISCARD/ : {
        *(.exception_vectors.*)
    }
}