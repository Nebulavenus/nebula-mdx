use crate::{attr};
use proc_macro2::{Span, TokenStream, Ident};
use quote::quote;
use syn::{
    Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Result,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => derive_struct(&input, fields),
        _ => Err(Error::new(
            Span::call_site(),
            "for now only structs with named fields are supported",
        )),
    }
}

pub fn derive_struct(input: &DeriveInput, fields: &FieldsNamed) -> Result<TokenStream> {
    // Struct name
    let structname = &input.ident;

    // Struct fields
    //let fieldname = fields.named.iter().map(|f| &f.ident);
    let fieldty = fields.named.iter().map(|f| {
        let field_ident = &f.ident;
        let ty = &f.ty;
        #[allow(unused_assignments)]
        let mut expr = quote! {};
        match *ty {
            // Only for [u8; size]?
            syn::Type::Array(ref array) => {
                if let syn::Type::Path(tp) = array.elem.as_ref() {
                    let _inner_ty = tp.path.get_ident().unwrap();
                    match array.len {
                        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref int), ..}) => {
                            let _size = int.base10_parse::<usize>().unwrap();
                            expr = quote! {
                                result += size_of_val(&self.#field_ident);
                            }
                        },
                        _ => expr = syn::Error::new_spanned(array, "NMread derive with bad array constexpr").to_compile_error(),
                    }
                }
            }
            syn::Type::Path(ref p) => {
                // All types without angle brackets
                if let Some(field_type) = p.path.get_ident() {
                    //dbg!(&field_type);
                    match field_type.to_string().as_str() {
                        "String" => {
                            // Max name len comes from #fieldty in final expand
                            expr = quote! { result += max_name_len; };
                        },
                        "u32" => {
                            expr = quote! { result += size_of_val(&self.#field_ident); };
                        },
                        "f32" => {
                            expr = quote! { result += size_of_val(&self.#field_ident); };
                        }
                        _ => {
                            expr = quote! { result += &self.#field_ident.total_bytes_size(); };
                        }
                    }
                } else {
                    //dbg!(&p.path);
                    let angle_ident = &p.path.segments[0].ident;
                    match angle_ident.to_string().as_str() {
                        "Vec" => {
                            if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args: val, ..}) = &p.path.segments[0].arguments {
                                //dbg!(val);
                                if let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, ..})) = val.iter().next().unwrap() {
                                    if let Some(_inner_type) =  path.get_ident() {

                                        // Check for vec_behaviour attribute tag and generate code suited for this type of behaviour
                                        if let Ok(Some(s)) = attr::vec_behaviour(f) {
                                            match s.as_str() {
                                                // For chunks
                                                "inclusive" => {
                                                    expr = quote! {
                                                        for val in &self.#field_ident {
                                                            result += val.total_bytes_size();
                                                        }
                                                    };
                                                },
                                                // For cases where first comes sequence_number, then array with the data
                                                "normal" => {
                                                    expr = quote! {
                                                        //result += &self.#field_ident.len();
                                                        result += size_of::<u32>();
                                                        for val in &self.#field_ident {
                                                            result += val.total_bytes_size();
                                                        }
                                                    };
                                                },
                                                _ => expr = syn::Error::new_spanned(f, format!("'{}' is unknown value for this attribute", s.as_str())).to_compile_error(),
                                            }
                                        }
                                    } else {
                                        expr = syn::Error::new_spanned(path, "Multiple angle brackets are not supported.").to_compile_error();
                                    }
                                }
                            }
                        },
                        _ => expr = syn::Error::new_spanned(angle_ident, format!("{} :unsupported angled type", angle_ident.to_string())).to_compile_error(),
                    }
                }
            },
            _ => expr = syn::Error::new_spanned(ty, "Is this struct or what?").to_compile_error(),
        }
        expr
    });
    
    let fieldtag = fields.named.iter().map(|f| {
        let mut expr = quote! {};
        if let Ok(Some(s)) = attr::tag_to_rw(f) {
            let _tag_ident = Ident::new(s.as_str(), Span::call_site());
            expr = quote! {
                result += 4;
            }
        }
        if let Ok(Some(s)) = attr::length_of_string(f) {
            if let Ok(length) = s.parse::<usize>() {
                expr = quote! {
                    let max_name_len = #length;
                }
            }
        }
        expr
    });

    // Increment result based on field bytes size in struct
    let items = quote! {
        #(
            #fieldtag
            #fieldty
        )*
    };

    Ok(quote! {
        impl BytesTotalSize for #structname {
            fn total_bytes_size(&self) -> usize {
                use std::mem::{size_of_val, size_of};
                let mut result = 0usize;

                #items

                result
            }
        }
    })
}