extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Poc)]
pub fn poc_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            fn poc() {
                crate::kprintln!("This is a poc for derive {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

