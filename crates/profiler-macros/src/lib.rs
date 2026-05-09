use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn profile(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let fn_name = &sig.ident;

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            let __profiler_guard =
                ::profiler::enter_scope(concat!(module_path!(), "::", stringify!(#fn_name)));
            let __profiler_guard = __profiler_guard;
            #block
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn profile_init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            let start_cpu = ::profiler::read_cycles().unwrap();
            #block
            let end_cpu = ::profiler::read_cycles().unwrap();
            ::profiler::print_sessions(start_cpu, end_cpu);

        }
    };

    expanded.into()
}
