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

/// Predefined const TAG: u32, like ``VERS_TAG``
/// #[nebula(tag = "<value>")]
pub fn tag_to_rw(field: &Field) -> Result<Option<String>> {
    let val = attr_value(&field.attrs, "tag")?;
    Ok(val)
}

/// Length for the string ``336``, ``80``
/// #[nebula(length = "<value>")]
pub fn length_of_string(field: &Field) -> Result<Option<String>> {
    let val = attr_value(&field.attrs, "length")?;
    Ok(val)
}

/// Posible values are ``inclusive``, ``normal``, ``divided``
/// #[nebula(behaviour = "<value>")]
pub fn vec_behaviour(field: &Field) -> Result<Option<String>> {
    let val = attr_value(&field.attrs, "behaviour")?;
    Ok(val)
}

/// Posible values are ``unknown_tag``, ``normal``
/// #[nebula(order = "<value>")]
pub fn option_order(field: &Field) -> Result<Option<String>> {
    let val = attr_value(&field.attrs, "order")?;
    Ok(val)
}
