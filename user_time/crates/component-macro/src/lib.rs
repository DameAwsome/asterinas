#![deny(unsafe_code)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn init_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let f = parse_macro_input!(item as ItemFn);
	let name = &f.sig.ident;
	let gen = quote! {
		#f
		const fn __file__() -> &'static str { file!() }
		component::submit!(component::ComponentRegistry::new(&#name, __file__()));
	};
	gen.into()
}

#[proc_macro]
pub fn parse_metadata(_item: TokenStream) -> TokenStream {
	quote! { ::std::vec::Vec::new() }.into()
}