#![feature(lang_items, no_std, linkage)]
#![no_std]

// Required lang items
// These definitions are marked "available_externally", which causes LLVM to discard
// the generated symbols.  The real ones are declared in weak_lang_items.s as weak
// aliases to "abort" (which is also a weak symbol).
// 
// 

#[lang="panic_fmt"] #[linkage = "available_externally"]
extern fn panic_fmt(_: &core::fmt::Arguments, _: &(&'static str, usize)) -> ! {
    loop {}
}

#[lang="stack_exhausted"] #[linkage = "available_externally"]
extern fn stack_exhausted() {}

#[lang="eh_personality"] #[linkage = "available_externally"]
extern fn rust_eh_personality() {}
