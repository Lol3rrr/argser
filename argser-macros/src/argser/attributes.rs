use std::fmt::Debug;

use proc_macro2::Ident;
use syn::{parenthesized, Token};

/// A single Argser-Attribute that is applied to a Field
#[derive(Debug)]
pub enum FieldAttribute {
    /// The Field is Marked as containing a SubCategory and should therefore
    /// be parsed not as a "Primitive"-Type but rather an entire category
    SubCategory,
    /// The Field should be parsed using a different Name
    Rename {
        /// The Name to use for parsing
        name: String,
    },
    /// The Field should use a Function, with the given Name, as fallback to
    /// generate the Default-Value
    DefaultFunc {
        /// The Name of the Fallback-Function
        func: Ident,
    },
    /// The Field should use the Types Default implementation as fallback
    Default_,
}

impl syn::parse::Parse for FieldAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let ident_str = ident.to_string();

        match ident_str.as_ref() {
            "subcategory" => Ok(FieldAttribute::SubCategory),
            "rename" => {
                let content;
                parenthesized!(content in input);

                let value: syn::LitStr = content.parse()?;

                Ok(FieldAttribute::Rename {
                    name: value.value(),
                })
            }
            "default_func" => {
                let content;
                parenthesized!(content in input);

                let value: Ident = content.parse()?;

                Ok(FieldAttribute::DefaultFunc { func: value })
            }
            "default" => Ok(FieldAttribute::Default_),
            _ => Err(syn::Error::new(ident.span(), "Unknown Attribute")),
        }
    }
}

#[derive(Debug)]
struct FieldAttributeList(Vec<FieldAttribute>);

impl syn::parse::Parse for FieldAttributeList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut result = Vec::new();
        let items = input.parse_terminated::<FieldAttribute, Token![,]>(FieldAttribute::parse)?;

        for item in items {
            result.push(item);
        }

        Ok(FieldAttributeList(result))
    }
}

pub fn parse_attributes(attributes: &[syn::Attribute]) -> Vec<FieldAttribute> {
    let mut result = Vec::new();

    for attr in attributes {
        match attr.style {
            syn::AttrStyle::Outer => {
                if !attr.path.is_ident("argser") {
                    continue;
                }

                let inner: FieldAttributeList = match attr.parse_args() {
                    Ok(i) => i,
                    Err(_) => {
                        continue;
                    }
                };

                result.extend(inner.0);
            }
            syn::AttrStyle::Inner(_) => continue,
        };
    }

    result
}
