#![feature(plugin_registrar)]
#![feature(quote)]
#![feature(rustc_private)]

// TODO: any way to reliably stash auto-generated items into a
// submodule or something, so the names don't pollute the current
// module's namespace?

extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;

#[plugin_registrar]
pub fn register(reg: &mut Registry) {
    use syntax::parse::token;
    use syntax::ext::base::MultiDecorator;
    
    reg.register_syntax_extension(
        token::intern("entry_point"),
        MultiDecorator(Box::new(entry_point::decorate)));
    
    reg.register_syntax_extension(
        token::intern("isr"),
        MultiDecorator(Box::new(isr::decorate)));
}

mod entry_point {
    // #[entry_point] annotation
    // Transform this:
    
        // #[entry_point]
        // pub fn main() -> ! {
        //     // ...
        // }
    
    // Into this:
    
        // #[start] #[linkage="available_externally"]
        // pub fn not_main(_: isize, _: *const *const u8) -> isize {
        //     0
        // }
        // 
        // #[link_name="start"]
        // pub extern fn main() -> ! {
        //     main()
        // }
        // 
        // fn main() -> ! {
        //     // ...
        // }
    
    use syntax::ast;
    use syntax::codemap::Span;
    use syntax::ext::base::{Annotatable, ExtCtxt};
    use syntax::ext::build::AstBuilder;
    
    pub fn decorate(
        cx:     &mut ExtCtxt,
        sp:     Span,
        _:      &ast::MetaItem,
        item:   &Annotatable,
        push:   &mut FnMut(Annotatable))
    {
        if let &Annotatable::Item(ref main) = item
        {
            generate_main(cx, sp, main.ident, push)
        }
        else
        {
            cx.span_err(sp, "entry_point must be a function");
        }
    }

    fn generate_main(
        cx:     &mut ExtCtxt,
        sp:     Span,
        main:   ast::Ident,
        push:   &mut FnMut(Annotatable))
    {
        // Satisfy rust's perverse desire to have an entry point of this type
        let not_main = quote_item!(cx,
            #[start] #[linkage = "available_externally"]
            fn __unused_this_is_not_main(_: isize, _: *const *const u8) -> isize {
                0
            }
        ).unwrap();
        
        let call_original = cx.expr_call_ident(sp, main, Vec::new());
        
        // Now provide a real entry point
        let start = quote_item!(cx,
            #[no_mangle]
            pub extern fn start() -> ! {
                $call_original
            }
        ).unwrap();
        
        push(Annotatable::Item(not_main));
        push(Annotatable::Item(start));
    }
}

mod isr {
    // Transform this:
    
        // #[isr(hard_fault, bus_fault)]
        // fn fault() {
        //     // Stuff...
        // }
    
    // Into this:
    
        // #[no_mangle]
        // fn hard_fault_handler() {
        //     fault()
        // }
        //
        // #[no_mangle]
        // fn bus_fault_handler() {
        //     fault()
        // }
        // 
        // fn fault() {
        //     // Stuff...
        // }
    
    use syntax::ast;
    use syntax::attr::AttrMetaMethods;
    use syntax::codemap::Span;
    use syntax::ext::base::{Annotatable, ExtCtxt};
    use syntax::ext::build::AstBuilder;
    use syntax::parse::token::InternedString;
    
    // TODO: sanity checks for instantiations
    // (check for duplicates, check against 
    // collection of known / registered vectors)
    
    pub fn decorate(
        cx:     &mut ExtCtxt,
        sp:     Span,
        meta:   &ast::MetaItem,
        item:   &Annotatable,
        push:   &mut FnMut(Annotatable))
    {
        // TODO: better validation and error reporting
        
        let interrupt_names = match meta.meta_item_list() {
            Some(isr_metas) => {
                isr_metas.into_iter()
                    .map(|m| m.name())
                    .collect::<Vec<InternedString>>()
            },
            
            None            => {
                cx.span_err(sp, "isr attribute requires params: for example, #[isr(bus_fault)]");
                return;
            }
        };
        
        let isr_item = match item {
            &Annotatable::Item(ref isr) => isr,
            _ => {
                cx.span_err(sp, "isr attribute applied to non-item");
                return;
            }
        };
        
        let call_original = cx.expr_call_ident(sp, isr_item.ident, Vec::new());
        
        for interrupt_name in interrupt_names {
            let isr_name = format!("{}_handler", interrupt_name);
            let isr_ident = cx.ident_of(&isr_name);
            
            let isr = quote_item!{cx,
                #[no_mangle]
                pub unsafe extern fn $isr_ident() {
                    $call_original
                }
            }.expect("quote_item! failed");
            
            push(Annotatable::Item(isr));
        }
    }
}
