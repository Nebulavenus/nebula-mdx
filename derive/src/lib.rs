#![recursion_limit="1024"]
extern crate proc_macro;
extern crate syn;

mod attr;

use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use syn::Token;
use syn::token::Token;
use syn::{parenthesized};

#[proc_macro_derive(NMread, attributes(nebula))]
pub fn derive_nmread(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_try_from_ctx(&ast);
    gen.into()
}

fn impl_try_from_ctx(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match ast.data {
        syn::Data::Struct(ref data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    impl_struct(name, fields)
                },
                _ => {
                    panic!("NMread can only be derived for a regular struct with public fields")
                }
            }
        },
        _ => panic!("NMread can only be derived for structs")
    }
}

fn nebula_of(attrs: &Vec<syn::Attribute>) -> Option<&syn::Attribute> {
    for attr in attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "nebula" {
            return Some(attr);
        }
    }
    None
}

enum Attribute {
    Tag(syn::Expr)
}

impl syn::parse::Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let key: syn::Ident = input.parse()?;
        let value = match key.to_string().as_str() {
            "tag" => {
                let _: Token![=] = input.parse()?;
                let expr: syn::Expr = input.parse()?;
                Attribute::Tag(expr)
            }
            key => panic!("Unrecognized attribute: '{}'", key)
        };

        Ok(value)
    }
}

struct Attributes(syn::punctuated::Punctuated<Attribute, Token![,]>);

impl syn::parse::Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(Attributes(content.parse_terminated(Attribute::parse)?))
    }
}

fn parse_tag(attr: &syn::Attribute) -> Option<proc_macro2::TokenStream> {
    fn mk_err<T: quote::ToTokens>(t: T) -> Option<proc_macro2::TokenStream> {
        Some(
            syn::Error::new_spanned(t, "expected `nebula(tag = \"...\")`").to_compile_error(),
        )
    }

    let meta = match attr.parse_meta() {
        Ok(syn::Meta::List(mut nvs)) => {
            // List is nebula(..)
            if nvs.nested.len() != 1 {
                return mk_err(nvs);
            }

            match nvs.nested.pop().unwrap().into_value() {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    if nv.path.get_ident().unwrap() != "tag" {
                        // Should be tag
                        return mk_err(nvs);
                    }
                    nv
                }
                meta => {
                    // was not k = v
                    return mk_err(meta);
                }
            }
        }
        Ok(meta) => {
            // was nebula = smt
            return mk_err(meta);
        }
        Err(e) => {
            return Some(e.to_compile_error());
        }
    };

    match meta.lit {
        syn::Lit::Int(i) => {
            let expect_value = i.base10_parse::<u32>().expect("Integer parsing error");

            let expr = quote! {
                let read_value = src.gread_with::<u32>(offset, ctx)?;
                if read_value != #expect_value {
                    let hex_read = format!("{:X}", read_value);
                    let hex_expect = format!("{:X}", expect_value);
                    panic!("MDXFormat is not correct, expected {}, found {}", hex_expect, hex_read);
                }
            };
            Some(expr)
        },
        lit => {
            panic!("expected int, found {:?}", lit)
        },
    }
}

fn impl_struct(name: &syn::Ident, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let items: Vec<_> = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let attrs = &f.attrs;
        let tag_exp1r = {
            if let Some(g) = nebula_of(attrs) {
                parse_tag(g).unwrap()
            } else {
                quote! { }
            }
        };

        let mut some_tag = None;
        {
            for attr in attrs {
                let path = &attr.path.segments[0].ident;
                if path == "nebula" {
                    let parsed_attrs: Attributes = syn::parse2(attr.tokens.clone()).unwrap();
                    for attr in parsed_attrs.0 {
                        match attr {
                            Attribute::Tag(expr) => some_tag = Some(expr),
                        }
                    }
                }
            }
        };
        dbg!(some_tag);

        match *ty {
            syn::Type::Array(ref array) => {
                match array.len {
                    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref int), ..}) => {
                        let size = int.base10_parse::<usize>().unwrap();
                        quote! {
                            #ident: {
                                //#tag_expr

                                let mut __tmp: #ty = [0; #size];
                                src.gread_inout_with(offset, &mut __tmp, ctx)?;
                                __tmp
                            }
                        }
                    },
                    _ => panic!("NMread derive with bad array constexpr")
                }
            },
            _ => {
                quote! {
                    #ident: {
                        //#tag_expr

                        src.gread_with::<#ty>(offset, ctx)?
                    }
                }
            }
        }
    }).collect();

    quote! {
        impl<'a> ::scroll::ctx::TryFromCtx<'a, ::scroll::Endian> for #name where #name: 'a {
            type Error = ::scroll::Error;
            #[inline]
            fn try_from_ctx(src: &'a [u8], ctx: ::scroll::Endian) -> ::scroll::export::result::Result<(Self, usize), Self::Error> {
                use ::scroll::Pread;
                let offset = &mut 0;
                let data  = #name { #(#items,)* };
                Ok((data, *offset))
            }
        }
    }
}
