#![recursion_limit="1024"]
extern crate proc_macro;
extern crate syn;

use quote::{format_ident, quote};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NMread)]
pub fn derive_mnread(input: TokenStream) -> TokenStream {
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

fn impl_struct(name: &syn::Ident, fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let items: Vec<_> = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        match *ty {
            syn::Type::Array(ref array) => {
                match array.len {
                    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref int), ..}) => {
                        let size = int.base10_parse::<usize>().unwrap();
                        quote! {
                            #ident: { let mut __tmp: #ty = [0; #size]; src.gread_inout_with(offset, &mut __tmp, ctx)?; __tmp }
                        }
                    },
                    _ => panic!("NMread derive with bad array constexpr")
                }
            },
            _ => {
                quote! {
                    #ident: src.gread_with::<#ty>(offset, ctx)?
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
