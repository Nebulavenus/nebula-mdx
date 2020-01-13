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
    let fieldty = fields.named.iter().map(|f| &f.ty);
    let fieldtag = fields.named.iter().map(|f| 
        if let Ok(Some(s)) = attr::tag_to_write(f) {
            let tag_ident = Ident::new(s.as_str(), Span::call_site());
            quote! {
                let read_tag = src.gread_with::<u32>(offset, ctx)?;
                assert_eq!(read_tag, #tag_ident as u32);
            }
        } else {
            quote! {}
        }
    );

    // Implement read methods for every field in struct
    let items = quote! {
        #(
            #fieldname: {
                #fieldtag
                //TODO: match #fieldty
                src.gread_with::<#fieldty>(offset, ctx)?
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