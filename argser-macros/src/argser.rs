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

impl ParseField {
    pub fn parse(field: &syn::Field) -> ParseField {
        let ident = field.ident.as_ref().unwrap().clone();
        let mut arg_name = ident.to_string();
        let mut ty = field.ty.clone();
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
                FieldAttribute::Map { sub } => {
                    let new_type: syn::Type =
                        syn::parse2(quote! { std::collections::HashMap<String, #ty> }).unwrap();

                    let sub_value = match sub {
                        Some(s) => match s.to_string().as_ref() {
                            "subcategory" => FieldValue::SubCategory,
                            _ => FieldValue::Primitive,
                        },
                        _ => FieldValue::Primitive,
                    };
                    value = FieldValue::Map(Box::new(sub_value), ty.clone());

                    ty = new_type;
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
        match &self.value {
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
            FieldValue::Map(sub, _) => {
                name.push('.');

                let ty = &self.ty;

                match sub.as_ref() {
                    FieldValue::Primitive => {
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

                                let mut result = <#ty>::new();

                                for (sub_name, sub_value) in sub_category {
                                    match argser::ParseFromArgs::parse(sub_value) {
                                        Ok(res) => {
                                            result.insert(sub_name, res);
                                        }
                                        _ => continue,
                                    };
                                }

                                result
                            }
                        }
                    }
                    FieldValue::SubCategory => {
                        quote! {
                            {
                                let mut sub_category = std::collections::HashMap::<String, std::collections::HashMap<String, Vec<String>>>::new();
                                for (key, value) in args.iter() {
                                    let n_key = match key.strip_prefix(#name) {
                                        Some(k) => k,
                                        None => continue,
                                    };

                                    let (name, sub_key) = match n_key.find('.') {
                                        Some(i) => (&n_key[..i], &n_key[(i+1)..]),
                                        None => continue,
                                    };


                                    match sub_category.get_mut(name) {
                                        Some(sub_value) => {
                                            sub_value.insert(sub_key.to_owned(), value.to_owned());
                                        }
                                        None => {
                                            let mut tmp = std::collections::HashMap::new();
                                            tmp.insert(sub_key.to_owned(), value.to_owned());

                                            sub_category.insert(name.to_owned(), tmp);
                                        }
                                    };
                                }

                                let mut result = <#ty>::new();

                                for (sub_name, sub_value) in sub_category {
                                    match argser::FromArgs::parse(sub_value) {
                                        Ok(res) => {
                                            result.insert(sub_name, res);
                                        }
                                        _ => continue,
                                    };
                                }

                                result
                            }
                        }
                    }
                    _ => panic!(""),
                }
            }
        }
    }
}

enum FieldValue {
    Primitive,
    SubCategory,
    Map(Box<FieldValue>, syn::Type),
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

fn impl_parse(input: &syn::ItemStruct, fields: &[ParseField]) -> TokenStream {
    let parse_block = generate_parse_block(fields);

    let mut fields = quote! {};
    for field in input.fields.iter() {
        let name = field.ident.as_ref().unwrap();
        fields.extend(quote! {
            #name,
        });
    }

    quote! {
        fn parse(args: std::collections::HashMap<String, Vec<String>>) -> Result<Self, argser::ParseError> {
            #parse_block

            Ok(Self {
                #fields
            })
        }
    }
}

fn impl_arguments(_input: &syn::ItemStruct, fields: &[ParseField]) -> TokenStream {
    let mut populate_block = quote! {};
    for field in fields {
        let name = &field.arg_name;

        match &field.value {
            FieldValue::Primitive => {
                let required = match &field.default_func {
                    DefaultValue::None => true,
                    DefaultValue::Impl | DefaultValue::Func(_) => false,
                };

                populate_block.extend(quote! {
                    args.push(argser::ArgumentDetail {
                        name: #name.to_owned(),
                        required: #required,
                        description: "".to_owned(),
                    });
                });
            }
            FieldValue::SubCategory => {
                let ty = &field.ty;

                populate_block.extend(quote! {
                    {
                        let raw = <#ty as argser::FromArgs>::arguments();
                        let extend_iter = raw
                            .into_iter()
                            .map(|mut raw| {
                                raw.name = format!("{}.{}", #name, raw.name);
                                raw
                            });
                        args.extend(extend_iter);
                    }
                });
            }
            FieldValue::Map(sub, prev_type) => {
                let block = match sub.as_ref() {
                    FieldValue::Primitive => quote! {
                        args.push(argser::ArgumentDetail {
                            name: format!("{}.{}", #name, "{name}"),
                            required: false,
                            description: "".to_owned(),
                        });
                    },
                    FieldValue::SubCategory => {
                        let ty = prev_type;
                        quote! {
                            let raw = <#ty as argser::FromArgs>::arguments();
                            let extend_iter = raw
                                .into_iter()
                                .map(|mut raw| {
                                    raw.name = format!("{}.{}.{}", #name, "{name}", raw.name);
                                    raw.required = false;
                                    raw
                                });
                            args.extend(extend_iter);
                        }
                    }
                    _ => quote! {},
                };

                populate_block.extend(block);
            }
        };
    }

    quote! {
        fn arguments() -> Vec<argser::ArgumentDetail> {
            let mut args = Vec::new();

            #populate_block

            args
        }
    }
}

fn impl_from_args(input: &syn::ItemStruct, fields: &[ParseField]) -> TokenStream {
    let name = &input.ident;

    let parse_block = impl_parse(input, fields);
    let arguments_block = impl_arguments(input, fields);

    quote! {
        impl argser::FromArgs for #name {
            #parse_block

            #arguments_block
        }
    }
}

fn generate_struct(input: &syn::ItemStruct, fields: &[ParseField]) -> TokenStream {
    let mut inner = input.clone();
    for field in inner.fields.iter_mut() {
        let name = field.ident.as_ref().unwrap();
        let n_type = fields
            .iter()
            .find(|nf| nf.ident.eq(name))
            .map(|nf| nf.ty.clone())
            .unwrap();

        let new_fields: Vec<_> = field
            .attrs
            .iter()
            .filter(|attr| {
                let path = &attr.path;
                let ident = match path.get_ident() {
                    Some(i) => i,
                    None => return false,
                };

                ident != "argser"
            })
            .map(|attr| attr.to_owned())
            .collect();

        field.attrs = new_fields;

        field.ty = n_type;
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
    let cleaned_up = generate_struct(&input, &fields);

    quote! {
        #cleaned_up

        #impl_block
    }
}
