#![allow(unused_imports)]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use quote::ToTokens;
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
	println!("This is from compilation");
	gen.into()
}

use proc_macro::*;
#[proc_macro_attribute]
pub fn test_case(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).unwrap();
	let fn_item = match &mut item {
		syn::Item::Fn(fn_item) => fn_item,
		_ => panic!("expected fn")
	};
	// Extract function name from ItemFn
	let fn_name = fn_item.sig.ident.to_string();

	// Create Block from code
	let function_head = quote! { crate::kprint!("{:40}", #fn_name); };
	fn_item.block.stmts.insert(0, syn::parse(function_head.into()).unwrap());
    let input: TokenStream = item.into_token_stream().into();

    let mut gen: TokenStream = quote!(#[test_case]).into_token_stream().into();
    gen.extend(input);
    gen
}



#[proc_macro_attribute]
pub fn poc_insertion(_args: TokenStream, input: TokenStream) -> TokenStream {
	// Extract function item from TokenStream
	let mut item: syn::Item = syn::parse(input).unwrap();
	let fn_item = match &mut item {
		syn::Item::Fn(fn_item) => fn_item,
		_ => panic!("expected fn")
	};
	// Extract function name from ItemFn
	let fn_name = fn_item.sig.ident.to_string();
	let args_name = _args.to_string();

	// Create Block from code
	let function_head = quote! {
		{
		crate::kprintln!("==================================");
		crate::kprintln!("Entering the function {}", #fn_name);
		crate::kprintln!("Received arg {}", #args_name);
		}
	};
	let function_tail = quote! {
		{
		crate::kprintln!("Leaving the function {}", #fn_name);
		crate::kprintln!("==================================");
		}
	};
	// stmts is a vec containing Block of code. Each blocks represent one line of code
	// Insert head block at start of the function blocks
	fn_item
		.block
		.stmts
		.insert(0, syn::parse(function_head.into()).unwrap());
	// Insert tail block at end of the function blocks
	fn_item.block.stmts.insert(
		fn_item.block.stmts.len(),
		syn::parse(function_tail.into()).unwrap()
	);

	// Convert back Item into TokenStream
	item.into_token_stream().into()
}

#[proc_macro]
pub fn proc_macro_poc(input: TokenStream) -> TokenStream {
	let code = quote! {
		fn proc_macro_poc() {
			crate::kprintln!("This is basic proc macro poc");
		}
	};
	code.into()
}
