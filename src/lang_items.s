.file "lang_items.s"

.include "weak_fn.s.h"

.section .text.abort
    .func abort
    weak_fn abort
    abort:
        b abort
    .size abort, . - abort
    .endfunc

    weak_fn_alias panic_fmt,            abort
    weak_fn_alias stack_exhausted,      abort
    weak_fn_alias rust_eh_personality,  abort
