//! # struct-mapper-derive
//!
//! Procedural macro crate for `struct-mapper`.
//! Provides `#[derive(MapFrom)]` to auto-generate `impl From<Source> for Target`.
//!
//! **Do not depend on this crate directly.** Use `struct-mapper` instead.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod codegen;
mod error;
mod field_match;
mod parse;

/// Derive macro that generates `impl From<Source> for Target` by mapping fields.
///
/// # Usage
///
/// ```rust,ignore
/// use struct_mapper::MapFrom;
///
/// struct UserEntity {
///     name: String,
///     email: String,
/// }
///
/// #[derive(MapFrom)]
/// #[map_from(UserEntity)]
/// struct UserResponse {
///     name: String,
///     email: String,
/// }
///
/// // Now you can do:
/// let entity = UserEntity { name: "Alice".into(), email: "a@b.com".into() };
/// let response: UserResponse = entity.into();
/// ```
///
/// # Field Attributes
///
/// - `#[map(from = "source_field")]` — Map from a differently-named source field
/// - `#[map(skip, default)]` — Skip this field, use `Default::default()`
/// - `#[map(into)]` — Call `.into()` on the source field value
/// - `#[map(with = "path::to::fn")]` — Apply a custom conversion function
#[proc_macro_derive(MapFrom, attributes(map_from, map))]
pub fn derive_map_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match codegen::expand_map_from(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
