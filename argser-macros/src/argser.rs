use std::fmt::Debug;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Type;

mod attributes;
pub use attributes::*;

enum DefaultValue {
    None,
    Impl,
    Func(Ident),
}

/// A single Field of a Struct that should be parsed
struct ParseField {
    ident: syn::Ident,
    ty: Type,
    arg_name: String,
    value: FieldValue,
    default_func: DefaultValue,
}
impl Debug for ParseField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParseField (ident: {}, type: {}, arg_name: {}, value: {:?})",
            self.ident,
            self.ty.to_token_stream(),
            self.arg_name,
            self.value
        )
    }
}

impl ParseField {
    pub fn parse(field: &syn::Field) -> ParseField {
        let ident = field.ident.as_ref().unwrap().clone();
        let mut arg_name = ident.to_string();
        let ty = field.ty.clone();
        let mut value = FieldValue::Primitive;
        let mut default_func = DefaultValue::None;

        let attributes = parse_attributes(&field.attrs);
        for attr in attributes {
            match attr {
                FieldAttribute::SubCategory => {
                    value = FieldValue::SubCategory;
                }
                FieldAttribute::Rename { name } => {
                    arg_name = name;
                }
                FieldAttribute::DefaultFunc { func } => {
                    default_func = DefaultValue::Func(func);
                }
                FieldAttribute::Default_ => {
                    default_func = DefaultValue::Impl;
                }
            };
        }

        ParseField {
            ident,
            ty,
            arg_name,
            value,
            default_func,
        }
    }

    pub fn parse_block(&self) -> TokenStream {
        let mut name = self.arg_name.clone();
        match self.value {
            FieldValue::Primitive => {
                let parse_fn = match &self.default_func {
                    DefaultValue::None => quote! { argser::ParseFromArgs::parse(value)? },
                    DefaultValue::Impl => {
                        quote! { argser::ParseFromArgs::parse_with_default(value) }
                    }
                    DefaultValue::Func(default_func_name) => {
                        quote! { argser::ParseFromArgs::parse_with_default_fn(value, #default_func_name) }
                    }
                };

                quote! {
                    {
                        let value = args.get(#name).map(|v| v.clone()).unwrap_or_else(|| Vec::new());

                        #parse_fn
                    }
                }
            }
            FieldValue::SubCategory => {
                name.push('.');
                quote! {
                    {
                        let mut sub_category = std::collections::HashMap::<String, Vec<String>>::new();
                        for (key, value) in args.iter() {
                            let n_key = match key.strip_prefix(#name) {
                                Some(k) => k,
                                None => continue,
                            };
                            sub_category.insert(n_key.to_owned(), value.to_owned());
                        }

                        argser::FromArgs::parse(sub_category)?
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum FieldValue {
    Primitive,
    SubCategory,
}

fn generate_parse_block(fields: &[ParseField]) -> TokenStream {
    let mut result = quote! {};
    for field in fields.iter() {
        let field_name = &field.ident;
        let parse_block = field.parse_block();
        result.extend(quote! {
            let #field_name = #parse_block;
        });
    }
    result
}

fn impl_from_args(input: &syn::ItemStruct, fields: &[ParseField]) -> TokenStream {
    let name = &input.ident;

    let parse_block = generate_parse_block(fields);

    let mut fields = quote! {};
    for field in input.fields.iter() {
        let name = field.ident.as_ref().unwrap();
        fields.extend(quote! {
            #name,
        });
    }

    quote! {
        impl argser::FromArgs for #name {
            fn parse(args: std::collections::HashMap<String, Vec<String>>) -> Result<Self, argser::ParseError> {
                #parse_block

                Ok(Self {
                    #fields
                })
            }
        }
    }
}

fn clean_up_struct(input: &syn::ItemStruct) -> TokenStream {
    let mut inner = input.clone();
    for field in inner.fields.iter_mut() {
        field.attrs = Vec::new();
    }

    inner.to_token_stream()
}

fn parse_fields(input: &syn::ItemStruct) -> Vec<ParseField> {
    input
        .fields
        .iter()
        .map(|field| ParseField::parse(field))
        .collect()
}

pub fn argser(_attributes: syn::AttributeArgs, input: syn::ItemStruct) -> TokenStream {
    let fields = parse_fields(&input);

    let impl_block = impl_from_args(&input, &fields);
    let cleaned_up = clean_up_struct(&input);

    quote! {
        #cleaned_up

        #impl_block
    }
}
