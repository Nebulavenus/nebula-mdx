use syn::{Attribute, Error, Field, Lit, Meta, NestedMeta, Result, parse_quote};

/// Find value of #[nebula(tag = "<expr>")] attribute
fn attr_value(attrs: &[Attribute]) -> Result<Option<String>> {
    let mut result = None;

    for attr in attrs {
        if !attr.path.is_ident("nebula") {
            continue;
        }

        let list = match attr.parse_meta()? {
            Meta::List(list) => list,
            other => return Err(Error::new_spanned(other, "unsupported attribute")),
        };

        for meta in &list.nested {
            if let NestedMeta::Meta(Meta::NameValue(value)) = meta {
                if value.path.is_ident("tag") {
                    if let Lit::Str(s) = &value.lit {
                        if result.is_some() {
                            return Err(Error::new_spanned(meta, "duplicate attribute"));
                        }
                        result = Some(s.value());
                        continue;
                    }
                }
            }
            return Err(Error::new_spanned(meta, "unsupported attribute"));
        }
    }
    Ok(result)
}

pub fn tag_to_write(field: &Field) -> Result<Option<String>> {
    let tag = attr_value(&field.attrs)?;
    Ok(tag)
}