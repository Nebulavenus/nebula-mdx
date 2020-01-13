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
            syn::Type::Array(ref _arr) => {
                unimplemented!();
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
                        _ => panic!("unknown type: {}", field_type.to_string().as_str()),
                    }
                    
                } else {
                    // Make work with Vec<>
                    unimplemented!();
                }
                //let type_str = format!("{:#?}", *ty);
                //dbg!(type_str);

                expr
            },
            _ => {
                panic!("Is this struct or what?");
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