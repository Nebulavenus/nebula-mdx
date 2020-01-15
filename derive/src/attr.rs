use syn::{Attribute, Error, Field, Lit, Meta, NestedMeta, Result};

/// Find value of #[nebula(<exval> = "<expr>")] attribute
fn attr_value(attrs: &[Attribute], exval: &str) -> Result<Option<String>> {
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
                if value.path.is_ident(exval) {
                    if let Lit::Str(s) = &value.lit {
                        if result.is_some() {
                            return Err(Error::new_spanned(meta, "duplicate attribute"));
                        }
                        result = Some(s.value());
                        continue;
                    }
                }
                continue;
            }
            return Err(Error::new_spanned(meta, "unsupported attribute"));
        }
    }
    Ok(result)
}

pub fn tag_to_rw(field: &Field) -> Result<Option<String>> {
    let val = attr_value(&field.attrs, "tag")?;
    Ok(val)
}

pub fn length_of_string(field: &Field) -> Result<Option<String>> {
    let val = attr_value(&field.attrs, "length")?;
    Ok(val)
}

pub fn vec_behaviour(field: &Field) -> Result<Option<String>> {
    let val = attr_value(&field.attrs, "behaviour")?;
    Ok(val)
}