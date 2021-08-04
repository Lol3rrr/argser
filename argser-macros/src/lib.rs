#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod argser;

/// This will automatically implement the `argser::FromArgs` trait for the
/// Struct it is applied on, while considering all the Configuration on the
/// Struct and all its parts when generating the implemenatation.
#[proc_macro_attribute]
pub fn argser(
    attributes: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: syn::AttributeArgs = syn::parse_macro_input!(attributes);
    let input: syn::ItemStruct = syn::parse_macro_input!(input);

    argser::argser(attr, input).into()
}
