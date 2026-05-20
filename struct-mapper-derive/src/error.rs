//! Helpful error messages with span information.
//!
//! Every error message follows this pattern:
//! 1. What went wrong
//! 2. Why it's wrong
//! 3. How to fix it

use proc_macro2::Span;

/// Create an error for when the derive is used on a non-struct type.
pub fn not_a_struct(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        "`#[derive(MapFrom)]` can only be used on structs with named fields.\n\n\
         Tuple structs, unit structs, and enums are not supported.\n\n\
         Example:\n  \
         #[derive(MapFrom)]\n  \
         #[map_from(Source)]\n  \
         struct Target {\n      \
             field: String,\n  \
         }",
    )
}
