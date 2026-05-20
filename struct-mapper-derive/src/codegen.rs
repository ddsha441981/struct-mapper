//! Code generation — the heart of the macro.
//!
//! Takes parsed struct info and generates `impl From<Source> for Target`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::error;
use crate::field_match::{resolve_field_mapping, FieldMapping};
use crate::parse::{parse_field_map_attr, parse_map_from_attr};

/// Main expansion entry point.
///
/// Parses the derive input, resolves field mappings, and generates the
/// `impl From<Source> for Target` block.
pub fn expand_map_from(input: &DeriveInput) -> syn::Result<TokenStream> {
    // 1. Parse #[map_from(SourceType)]
    let map_from_attr = parse_map_from_attr(&input.attrs)?;
    let source_type = &map_from_attr.source;
    let target_type = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // 2. Ensure it's a struct with named fields
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => return Err(error::not_a_struct(input.ident.span())),
        },
        _ => return Err(error::not_a_struct(input.ident.span())),
    };

    // 3. Generate field assignment expressions
    let mut field_assignments = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().ok_or_else(|| {
            error::not_a_struct(input.ident.span())
        })?;

        // Parse #[map(...)] attributes on this field
        let field_attr = parse_field_map_attr(field)?;

        // Resolve how this field should be mapped
        let mapping = resolve_field_mapping(field_name, &field_attr)?;

        let assignment = match mapping {
            FieldMapping::Direct { source_field } => {
                quote! { #field_name: source.#source_field }
            }
            FieldMapping::Renamed { source_field } => {
                quote! { #field_name: source.#source_field }
            }
            FieldMapping::Skipped => {
                quote! { #field_name: ::core::default::Default::default() }
            }
            FieldMapping::IntoConvert { source_field } => {
                quote! { #field_name: source.#source_field.into() }
            }
            FieldMapping::WithFunc {
                source_field,
                func_path,
            } => {
                let func: syn::ExprPath = syn::parse_str(&func_path).map_err(|_| {
                    syn::Error::new(
                        field_name.span(),
                        format!(
                            "invalid function path `{func_path}` in `#[map(with = \"...\")]`.\n\n\
                             The path must be a valid Rust expression like:\n  \
                             - `ToString::to_string`\n  \
                             - `|v| v as i32`\n  \
                             - `my_module::convert`"
                        ),
                    )
                })?;
                quote! { #field_name: #func(source.#source_field) }
            }
        };

        field_assignments.push(assignment);
    }

    // 4. Generate the impl block
    let expanded = quote! {
        impl #impl_generics ::core::convert::From<#source_type> for #target_type #ty_generics #where_clause {
            fn from(source: #source_type) -> Self {
                Self {
                    #(#field_assignments),*
                }
            }
        }
    };

    Ok(expanded)
}
