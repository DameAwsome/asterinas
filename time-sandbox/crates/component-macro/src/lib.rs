#![deny(unsafe_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn init_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let expanded = quote! {
        #input
        const fn file() -> &'static str { file!() }
        component::submit!(component::ComponentRegistry::new(&#name, file()));
    };
    TokenStream::from(expanded)
}

