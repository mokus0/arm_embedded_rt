
// declare a thumb function with weak linkage
.macro weak_fn alias
    .thumb_func
    .globl \alias
    .weak  \alias
.endm

// create a new function symbol called \alias,
// whose value is \orig but has weak linkage
.macro weak_fn_alias alias, orig
    weak_fn \alias
    \alias = \orig
.endm
