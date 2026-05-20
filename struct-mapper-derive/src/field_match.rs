//! Field matching logic — determines how source fields map to target fields.

use proc_macro2::Span;
use syn::Ident;

use crate::parse::FieldMapAttr;

/// Represents how a single target field should be populated.
#[derive(Debug)]
pub enum FieldMapping {
    /// Direct: `target.field = source.field` (same name, same type)
    Direct { source_field: Ident },
    /// Renamed: `target.field = source.other_name`
    Renamed { source_field: Ident },
    /// Skipped: `target.field = Default::default()`
    Skipped,
    /// Into: `target.field = source.field.into()`
    IntoConvert { source_field: Ident },
    /// Custom function: `target.field = func(source.field)`
    WithFunc {
        source_field: Ident,
        func_path: String,
    },
}

/// Resolve how a target field should be populated from the source.
pub fn resolve_field_mapping(
    target_field_name: &Ident,
    attr: &FieldMapAttr,
) -> syn::Result<FieldMapping> {
    // Case 1: Skip — use default
    if attr.skip {
        return Ok(FieldMapping::Skipped);
    }

    // Determine source field name (renamed or same)
    let source_field = if let Some(ref renamed) = attr.from {
        Ident::new(renamed, Span::call_site())
    } else {
        target_field_name.clone()
    };

    // Case 2: Custom function
    if let Some(ref func) = attr.with {
        return Ok(FieldMapping::WithFunc {
            source_field,
            func_path: func.clone(),
        });
    }

    // Case 3: Into conversion
    if attr.into {
        return Ok(FieldMapping::IntoConvert { source_field });
    }

    // Case 4: Direct mapping (renamed or same name)
    if attr.from.is_some() {
        Ok(FieldMapping::Renamed { source_field })
    } else {
        Ok(FieldMapping::Direct { source_field })
    }
}
