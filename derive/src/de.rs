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
    let fieldname = fields.named.iter().map(|f| &f.ident);
    let fieldty = fields.named.iter().map(|f| {
        let ty = &f.ty;
        match *ty {
            // Only for [u8; size]?
            syn::Type::Array(ref array) => {
                match array.len {
                    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(ref int), ..}) => {
                        let size = int.base10_parse::<usize>().unwrap();
                        quote! {
                            let mut tmp: #ty = [0; #size];
                            src.gread_inout_with(offset, &mut tmp, ctx)?; 
                            tmp
                        }
                    },
                    _ => syn::Error::new_spanned(array, "NMread derive with bad array constexpr").to_compile_error(),
                }
            }
            syn::Type::Path(ref p) => {
                #[allow(unused_assignments)]
                let mut expr = quote! {};
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
                            }
                        },
                        "u32" => {
                            expr = quote! { src.gread_with::<#ty>(offset, ctx)? }
                        },
                        _ => expr = syn::Error::new_spanned(field_type, format!("'{}' this type is not supported", field_type.to_string())).to_compile_error(),
                    }
                } else {
                    //dbg!(&p.path);
                    let angle_ident = &p.path.segments[0].ident;
                    match angle_ident.to_string().as_str() {
                        "Vec" => {
                            if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args: val, ..}) = &p.path.segments[0].arguments {
                                //dbg!(val);
                                if let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, ..})) = val.iter().next().unwrap() {
                                    if let Some(inner_type) =  path.get_ident() {
                                        //dbg!(inner_type);

                                        expr = quote! {
                                            // Right here comes vec_behaviour from #fieldtag
                                            match vec_behaviour {
                                                "inclusive" => {
                                                    let mut data = Vec::new();
                                                    let mut total_size = 0u32;
                                                    while total_size < Self.chunk_size {
                                                        let val = src.gread_with::<#inner_type>(offset, ctx)?;
                                                        total_size += val.inclusive_size;
                                                        data.push(val);
                                                    }
                                                },
                                                "normal" => {
                                                    let values_count = src.gread_with::<u32>(offset, ctx)?;
                                                    let mut values = Vec::<#inner_type>::with_capacity(values_count);
                                                    for _ in 0..values_count {
                                                        let value = src.gread_with::<#inner_type>(offset, ctx)?;
                                                        values.push(value);
                                                    }
                                                    values
                                                },
                                                _ => unreachable!(),
                                            }
                                        };
                                    } else {
                                        expr = syn::Error::new_spanned(path, "Multiple angle brackets are not supported.").to_compile_error();
                                    }
                                }
                            }
                        },
                        _ => expr = syn::Error::new_spanned(angle_ident, format!("{} :unsupported angled type", angle_ident.to_string())).to_compile_error(),
                    }
                }

                expr
            },
            _ => {
                syn::Error::new_spanned(ty, "Is this struct or what?").to_compile_error()
            }
        }
    });
    
    let fieldtag = fields.named.iter().map(|f| {
        let mut expr = quote! {};
        if let Ok(Some(s)) = attr::tag_to_rw(f) {
            let tag_ident = Ident::new(s.as_str(), Span::call_site());
            expr = quote! {
                let read_tag = src.gread_with::<u32>(offset, ctx)?;
                assert_eq!(read_tag, #tag_ident as u32);
            }
        }
        if let Ok(Some(s)) = attr::length_of_string(f) {
            if let Ok(length) = s.parse::<usize>() {
                expr = quote! {
                    let max_name_len = #length;
                }
            }
        }
        if let Ok(Some(s)) = attr::vec_behaviour(f) {
            match s.as_str() {
                // For chunks
                "inclusive" => {
                    expr = quote! { let vec_behaviour = #s; };
                },
                // For cases where first comes sequence_number, then array with the data
                "normal" => {
                    expr = quote! { let vec_behaviour = #s; };
                },
                _ => expr = syn::Error::new_spanned(f, format!("'{}' is unknown value for this attribute", s.as_str())).to_compile_error(),
            }
        }
                                        
        expr
    });

    // Implement read methods for every field in struct
    let items = quote! {
        #(
            #fieldname: {
                #fieldtag
                #fieldty
            },
        )*
    };

    Ok(quote! {
        impl<'a> ::scroll::ctx::TryFromCtx<'a, ::scroll::Endian> for #structname where #structname: 'a {
            type Error = ::scroll::Error;
            #[inline]
            fn try_from_ctx(src: &'a [u8], ctx: ::scroll::Endian) -> ::scroll::export::result::Result<(Self, usize), Self::Error> {
                use ::scroll::Pread;
                let offset = &mut 0;
                let data  = #structname { #items };
                Ok((data, *offset))
            }
        }
    })
}