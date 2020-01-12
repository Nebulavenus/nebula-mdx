use crate::{attr};
use proc_macro2::{Span, TokenStream};
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
    let _fieldtag = fields
        .named
        .iter()
        .map(attr::tag_to_write)
        .collect::<Result<Vec<_>>>()?;

    // Implement read methods for every field in struct
    let items = quote! {
        #(
            #fieldname: {
                //TODO: match #fieldty, check for #fieldtag
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