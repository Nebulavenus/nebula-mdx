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
    let fieldstructname = fields.named.iter().map(|f| &f.ident);
    let fieldname = fields.named.iter().map(|f| &f.ident);
    let fieldty = fields.named.iter().map(|f| {
        let ty = &f.ty;
        #[allow(unused_assignments)]
        let mut expr = quote! {};
        match *ty {
            // Only for [u8; size]?
            syn::Type::Array(ref array) => {
                if let syn::Type::Path(tp) = array.elem.as_ref() {
                    let inner_ty = tp.path.get_ident().unwrap();
                    match array.len {
                        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref int), ..}) => {
                            let size = int.base10_parse::<usize>().unwrap();
                            expr = quote! {
                                let mut tmp: #ty = [<#inner_ty>::default(); #size];
                                src.gread_inout_with(offset, &mut tmp, ctx)?;
                                tmp
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
                            expr = quote! {
                                let name = src.gread::<&str>(&mut offset.clone())?.to_string();
                                *offset += max_name_len;
                                name
                            };
                        },
                        // Every single possible type, that also implements pread
                        _ => {
                            expr = quote! { src.gread_with::<#ty>(offset, ctx)? };
                        }
                    }
                } else {
                    //dbg!(&p.path);
                    //let angle_ident = &p.path.segments[0].ident;
                    // Angled brackets support only for ("Vec" | "Option")
                    if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args: val, ..}) = &p.path.segments[0].arguments {
                        //dbg!(val);
                        if let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, ..})) = val.iter().next().unwrap() {
                            if let Some(inner_type) =  path.get_ident() {

                                // For Vec<#inner_type>
                                // Check for vec_behaviour attribute tag and generate code suited for this type of behaviour
                                if let Ok(Some(s)) = attr::vec_behaviour(f) {
                                    match s.as_str() {
                                        // For cases like SEQS -> chunk_size / 132
                                        "divided" => {
                                            expr = quote! {
                                                // Get default total bytes size for this type
                                                let bts = <#inner_type>::default().total_bytes_size() as u32;

                                                let mut data = Vec::new();
                                                if let Some(values_count) = u32::checked_div(chunk_size.clone(), bts) {
                                                    for _ in 0..values_count {
                                                        let val = src.gread_with::<#inner_type>(offset, ctx)?;
                                                        data.push(val);
                                                    }
                                                }
                                                data
                                            }
                                        }
                                        // For chunks
                                        "inclusive" => {
                                            expr = quote! {
                                                let mut data = Vec::new();
                                                let mut total_size = 0u32;
                                                // Previous value
                                                while total_size < chunk_size {
                                                    let val = src.gread_with::<#inner_type>(offset, ctx)?;
                                                    total_size += val.inclusive_size;
                                                    data.push(val);
                                                }
                                                data
                                            };
                                        },
                                        // For cases where first comes sequence_number, then array with the data
                                        "normal" => {
                                            expr = quote! {
                                                let values_count = src.gread_with::<u32>(offset, ctx)?;
                                                let mut values = Vec::<#inner_type>::with_capacity(values_count as usize);
                                                for _ in 0..values_count {
                                                    let value = src.gread_with::<#inner_type>(offset, ctx)?;
                                                    values.push(value);
                                                }
                                                values
                                            };
                                        },
                                        _ => expr = syn::Error::new_spanned(f, format!("'{}' is unknown value for this attribute", s.as_str())).to_compile_error(),
                                    }
                                }

                                // For Option<#inner_type>
                                // Check for option_order attribute tag and generate code suited for this type
                                if let Ok(Some(s)) = attr::option_order(f) {
                                    match s.as_str() {
                                        "unknown_tag" => {
                                            expr = quote! {
                                                let mut result: Option<#inner_type> = None;
                                                if (*offset as u32) < inclusive_size {
                                                    // Reread previous tag
                                                    *offset -= 4;
                                                    let tag = src.gread_with::<u32>(offset, ctx).unwrap();
                                                    if tag == expect_tag {
                                                        let value = src.gread_with::<#inner_type>(offset, ctx)?;
                                                        result = Some(value);
                                                    }
                                                }
                                                result
                                            };
                                        },
                                        "normal" => {

                                        },
                                        _ => expr = syn::Error::new_spanned(f, format!("'{}' is unknown value for this attribute", s.as_str())).to_compile_error(),
                                    }
                                }
                            } else {
                                expr = syn::Error::new_spanned(path, "Multiple angle brackets are not supported.").to_compile_error();
                            }
                        }
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
            let tag_ident = Ident::new(s.as_str(), Span::call_site());
            expr = quote! {
                let expect_tag = src.gread_with::<u32>(offset, ctx)?;
                assert_eq!(expect_tag, #tag_ident as u32);
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

    // Implement read methods for every field in struct
    let items = quote! {
        #(
            let #fieldname = {
                #fieldtag
                #fieldty
            };
        )*
    };

    Ok(quote! {
        impl<'a> ::scroll::ctx::TryFromCtx<'a, ::scroll::Endian> for #structname where #structname: 'a {
            type Error = ::scroll::Error;
            #[inline]
            fn try_from_ctx(src: &'a [u8], ctx: ::scroll::Endian) -> ::scroll::export::result::Result<(Self, usize), Self::Error> {
                use ::scroll::Pread;
                let offset = &mut 0;

                #items

                let result  = #structname { #(#fieldstructname,)* };
                Ok((result, *offset))
            }
        }
    })
}