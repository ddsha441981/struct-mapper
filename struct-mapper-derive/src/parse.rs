//! Parsing logic for `#[map_from(...)]` and `#[map(...)]` attributes.

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Field, LitStr, Token,
};

/// Parsed data from `#[map_from(SourceType)]` on the struct.
#[derive(Debug)]
pub struct MapFromAttr {
    /// The source type to map from.
    pub source: syn::Path,
}

impl Parse for MapFromAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let source = input.parse::<syn::Path>()?;
        Ok(MapFromAttr { source })
    }
}

/// Parsed data from `#[map(...)]` on individual fields.
#[derive(Debug, Default)]
pub struct FieldMapAttr {
    /// Rename: map from a differently-named source field.
    pub from: Option<String>,
    /// Skip this field (don't map from source).
    pub skip: bool,
    /// Use `Default::default()` for skipped fields.
    pub default: bool,
    /// Call `.into()` on the source value.
    pub into: bool,
    /// Custom conversion function path.
    pub with: Option<String>,
    /// Call `.try_into()` on the source value (fallible).
    pub try_into: bool,
    /// Fallible custom conversion function path.
    pub try_with: Option<String>,
}

/// Parse the `#[map_from(SourceType)]` attribute from a struct's attributes.
pub fn parse_map_from_attr(attrs: &[Attribute]) -> syn::Result<MapFromAttr> {
    for attr in attrs {
        if attr.path().is_ident("map_from") {
            return attr.parse_args::<MapFromAttr>();
        }
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "missing `#[map_from(SourceType)]` attribute.\n\
         Add `#[map_from(YourSourceStruct)]` to specify which struct to map from.\n\n\
         Example:\n  \
         #[derive(MapFrom)]\n  \
         #[map_from(UserEntity)]\n  \
         struct UserResponse { ... }",
    ))
}

/// Parse the `#[try_map_from(SourceType)]` attribute from a struct's attributes.
pub fn parse_try_map_from_attr(attrs: &[Attribute]) -> syn::Result<MapFromAttr> {
    for attr in attrs {
        if attr.path().is_ident("try_map_from") {
            return attr.parse_args::<MapFromAttr>();
        }
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "missing `#[try_map_from(SourceType)]` attribute.\n\
         Add `#[try_map_from(YourSourceStruct)]` to specify which struct to map from.\n\n\
         Example:\n  \
         #[derive(TryMapFrom)]\n  \
         #[try_map_from(UserEntity)]\n  \
         struct UserResponse { ... }",
    ))
}

/// Keyword-like tokens used in `#[map(...)]` attributes.
mod kw {
    syn::custom_keyword!(from);
    syn::custom_keyword!(skip);
    syn::custom_keyword!(default);
    syn::custom_keyword!(into);
    syn::custom_keyword!(with);
    syn::custom_keyword!(try_into);
    syn::custom_keyword!(try_with);
}

/// A single key=value or flag inside `#[map(...)]`.
enum MapOption {
    From(String),
    Skip,
    Default,
    Into,
    With(String),
    TryInto,
    TryWith(String),
}

impl Parse for MapOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::from) {
            input.parse::<kw::from>()?;
            input.parse::<Token![=]>()?;
            let lit: LitStr = input.parse()?;
            Ok(MapOption::From(lit.value()))
        } else if lookahead.peek(kw::skip) {
            input.parse::<kw::skip>()?;
            Ok(MapOption::Skip)
        } else if lookahead.peek(kw::default) {
            input.parse::<kw::default>()?;
            Ok(MapOption::Default)
        } else if lookahead.peek(kw::into) {
            input.parse::<kw::into>()?;
            Ok(MapOption::Into)
        } else if lookahead.peek(kw::with) {
            input.parse::<kw::with>()?;
            input.parse::<Token![=]>()?;
            let lit: LitStr = input.parse()?;
            Ok(MapOption::With(lit.value()))
        } else if lookahead.peek(kw::try_into) {
            input.parse::<kw::try_into>()?;
            Ok(MapOption::TryInto)
        } else if lookahead.peek(kw::try_with) {
            input.parse::<kw::try_with>()?;
            input.parse::<Token![=]>()?;
            let lit: LitStr = input.parse()?;
            Ok(MapOption::TryWith(lit.value()))
        } else {
            Err(lookahead.error())
        }
    }
}

/// Parse `#[map(...)]` attributes on a field.
pub fn parse_field_map_attr(field: &Field) -> syn::Result<FieldMapAttr> {
    let mut result = FieldMapAttr::default();

    for attr in &field.attrs {
        if !attr.path().is_ident("map") {
            continue;
        }

        let options: Punctuated<MapOption, Token![,]> =
            attr.parse_args_with(Punctuated::parse_terminated)?;

        for opt in options {
            match opt {
                MapOption::From(name) => result.from = Some(name),
                MapOption::Skip => result.skip = true,
                MapOption::Default => result.default = true,
                MapOption::Into => result.into = true,
                MapOption::With(func) => result.with = Some(func),
                MapOption::TryInto => result.try_into = true,
                MapOption::TryWith(func) => result.try_with = Some(func),
            }
        }
    }

    // Validation: skip requires default
    if result.skip && !result.default {
        let span = field
            .ident
            .as_ref()
            .map(|i| i.span())
            .unwrap_or_else(proc_macro2::Span::call_site);

        return Err(syn::Error::new(
            span,
            "`#[map(skip)]` requires `#[map(skip, default)]`.\n\
             When skipping a field, you must provide a default value.\n\n\
             Fix: #[map(skip, default)]",
        ));
    }

    // Validation: from + skip is contradictory
    if result.from.is_some() && result.skip {
        let span = field
            .ident
            .as_ref()
            .map(|i| i.span())
            .unwrap_or_else(proc_macro2::Span::call_site);

        return Err(syn::Error::new(
            span,
            "`#[map(from = \"...\")]` and `#[map(skip)]` are contradictory.\n\
             A field cannot be both mapped from a source and skipped.",
        ));
    }

    // Validation: into + try_into is contradictory
    if result.into && result.try_into {
        let span = field
            .ident
            .as_ref()
            .map(|i| i.span())
            .unwrap_or_else(proc_macro2::Span::call_site);

        return Err(syn::Error::new(
            span,
            "`#[map(into)]` and `#[map(try_into)]` are contradictory.\n\
             Use `into` for infallible conversions or `try_into` for fallible ones, not both.",
        ));
    }

    // Validation: with + try_with is contradictory
    if result.with.is_some() && result.try_with.is_some() {
        let span = field
            .ident
            .as_ref()
            .map(|i| i.span())
            .unwrap_or_else(proc_macro2::Span::call_site);

        return Err(syn::Error::new(
            span,
            "`#[map(with = \"...\")]` and `#[map(try_with = \"...\")]` are contradictory.\n\
             Use `with` for infallible functions or `try_with` for fallible ones, not both.",
        ));
    }

    Ok(result)
}
