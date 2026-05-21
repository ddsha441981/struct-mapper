//! # struct-mapper
//!
//! **Derive macro to auto-generate `impl From<Source> for Target` by mapping struct fields.**
//!
//! Stop writing tedious manual `From` implementations for struct-to-struct conversions.
//! `struct-mapper` generates them at compile time with **zero runtime overhead**.
//!
//! ---
//!
//! ## Overview
//!
//! In Rust backend development with frameworks like Axum, Actix, or Rocket, you constantly
//! need to convert between different struct representations — database entities to API responses,
//! form inputs to domain models, and so on. This leads to dozens of repetitive `impl From<A> for B`
//! blocks that are tedious to write and maintain.
//!
//! `struct-mapper` eliminates this boilerplate with a single `#[derive(MapFrom)]` annotation.
//!
//! ## Quick Start
//!
//! ```rust
//! use struct_mapper::MapFrom;
//!
//! // Your source struct (e.g., from database layer)
//! struct UserEntity {
//!     name: String,
//!     email: String,
//!     age: u32,
//! }
//!
//! // Target struct — From<UserEntity> is auto-generated!
//! #[derive(Debug, MapFrom)]
//! #[map_from(UserEntity)]
//! struct UserResponse {
//!     name: String,
//!     email: String,
//!     age: u32,
//! }
//!
//! let entity = UserEntity {
//!     name: "Khushi".to_string(),
//!     email: "khushi@gmail.com".to_string(),
//!     age: 30,
//! };
//!
//! // Zero-boilerplate conversion!
//! let response: UserResponse = entity.into();
//! assert_eq!(response.name, "Khushi");
//! assert_eq!(response.email, "khushi@gmail.com");
//! assert_eq!(response.age, 30);
//! ```
//!
//! ---
//!
//! ## Feature Guide
//!
//! ### 1. Basic Mapping — Same Name, Same Type
//!
//! When source and target fields have the **same name and same type**, no extra
//! annotation is needed. The macro maps them automatically:
//!
//! ```rust
//! use struct_mapper::MapFrom;
//!
//! struct Source { name: String, age: u32 }
//!
//! #[derive(MapFrom)]
//! #[map_from(Source)]
//! struct Target { name: String, age: u32 }
//!
//! let t: Target = Source { name: "Deendayal".into(), age: 25 }.into();
//! assert_eq!(t.name, "Deendayal");
//! assert_eq!(t.age, 25);
//! ```
//!
//! ### 2. Field Renaming — `#[map(from = "...")]`
//!
//! When source and target use **different field names**, use `from` to specify
//! which source field to read from:
//!
//! ```rust
//! use struct_mapper::MapFrom;
//!
//! struct DbRow {
//!     user_name: String,
//!     user_age: u32,
//! }
//!
//! #[derive(MapFrom)]
//! #[map_from(DbRow)]
//! struct ApiUser {
//!     #[map(from = "user_name")]
//!     name: String,
//!     #[map(from = "user_age")]
//!     age: u32,
//! }
//!
//! let row = DbRow { user_name: "Anjali".into(), user_age: 35 };
//! let user: ApiUser = row.into();
//! assert_eq!(user.name, "Anjali");
//! assert_eq!(user.age, 35);
//! ```
//!
//! ### 3. Skip + Default — `#[map(skip, default)]`
//!
//! For target fields that **don't exist in the source**, mark them as skipped.
//! They will be filled with [`Default::default()`]:
//!
//! ```rust
//! use struct_mapper::MapFrom;
//!
//! struct Entity { name: String }
//!
//! #[derive(MapFrom)]
//! #[map_from(Entity)]
//! struct Response {
//!     name: String,
//!     #[map(skip, default)]
//!     request_id: String,    // → "" (String::default())
//!     #[map(skip, default)]
//!     retry_count: u32,      // → 0 (u32::default())
//! }
//!
//! let r: Response = Entity { name: "Nikhil".into() }.into();
//! assert_eq!(r.name, "Nikhil");
//! assert_eq!(r.request_id, "");
//! assert_eq!(r.retry_count, 0);
//! ```
//!
//! > **Note:** `skip` always requires `default`. Using `#[map(skip)]` alone
//! > will produce a clear compile error telling you to add `default`.
//!
//! ### 4. Nested Conversion — `#[map(into)]`
//!
//! When a source field's type implements `Into<TargetFieldType>`, use `into`
//! to automatically call `.into()` during conversion:
//!
//! ```rust
//! use struct_mapper::MapFrom;
//!
//! struct AddressEntity { city: String }
//!
//! #[derive(Debug, PartialEq, MapFrom)]
//! #[map_from(AddressEntity)]
//! struct AddressDTO { city: String }
//!
//! struct OrderEntity {
//!     id: u64,
//!     address: AddressEntity,
//! }
//!
//! #[derive(Debug, MapFrom)]
//! #[map_from(OrderEntity)]
//! struct OrderDTO {
//!     id: u64,
//!     #[map(into)]
//!     address: AddressDTO,  // source.address.into()
//! }
//!
//! let order = OrderEntity {
//!     id: 42,
//!     address: AddressEntity { city: "Springfield".into() },
//! };
//! let dto: OrderDTO = order.into();
//! assert_eq!(dto.id, 42);
//! assert_eq!(dto.address.city, "Springfield");
//! ```
//!
//! ### 5. Custom Function — `#[map(with = "...")]`
//!
//! For complex transformations, pass a **function path** that takes the source
//! field value and returns the target field type:
//!
//! ```rust
//! use struct_mapper::MapFrom;
//!
//! fn cents_to_dollars(cents: u64) -> f64 {
//!     cents as f64 / 100.0
//! }
//!
//! struct PriceEntity { amount_cents: u64 }
//!
//! #[derive(MapFrom)]
//! #[map_from(PriceEntity)]
//! struct PriceDTO {
//!     #[map(from = "amount_cents", with = "cents_to_dollars")]
//!     amount: f64,
//! }
//!
//! let dto: PriceDTO = PriceEntity { amount_cents: 1999 }.into();
//! assert!((dto.amount - 19.99).abs() < f64::EPSILON);
//! ```
//!
//! ### 6. Combining Attributes
//!
//! All field attributes can be **combined freely**:
//!
//! ```rust
//! use struct_mapper::MapFrom;
//!
//! fn to_upper(s: String) -> String { s.to_uppercase() }
//!
//! struct Source {
//!     id: u64,
//!     user_name: String,
//!     raw_email: String,
//! }
//!
//! #[derive(Debug, MapFrom)]
//! #[map_from(Source)]
//! struct Target {
//!     id: u64,                                       // direct
//!     #[map(from = "user_name")]
//!     name: String,                                  // renamed
//!     #[map(from = "raw_email", with = "to_upper")]
//!     email: String,                                 // renamed + custom fn
//!     #[map(skip, default)]
//!     request_id: String,                            // skipped
//! }
//!
//! let t: Target = Source {
//!     id: 1,
//!     user_name: "Sulochan".into(),
//!     raw_email: "sulochan@gmail.com".into(),
//! }.into();
//!
//! assert_eq!(t.id, 1);
//! assert_eq!(t.name, "Sulochan");
//! assert_eq!(t.email, "SULOCHAN@GMAIL.COM");
//! assert_eq!(t.request_id, "");
//! ```
//!
//! ---
//!
//! ## Attribute Reference
//!
//! ### Struct-Level
//!
//! | Attribute | Required | Description |
//! |:----------|:--------:|:------------|
//! | `#[map_from(Type)]` | **Yes** | Specifies the source type for `From<Type>` generation |
//!
//! ### Field-Level
//!
//! | Attribute | Description |
//! |:----------|:------------|
//! | `#[map(from = "name")]` | Map from a differently-named source field |
//! | `#[map(skip, default)]` | Skip this field; use `Default::default()` |
//! | `#[map(into)]` | Call `.into()` on the source field value |
//! | `#[map(with = "path")]` | Apply a conversion function `fn(SourceFieldType) -> TargetFieldType` |
//!
//! Attributes can be combined: `#[map(from = "old", with = "convert")]`
//!
//! ---
//!
//! ## Error Messages
//!
//! `struct-mapper` provides clear, actionable compile errors:
//!
//! - **Missing `#[map_from]`:** Tells you exactly which attribute to add, with an example.
//! - **`#[map(skip)]` without `default`:** Shows the fix: `#[map(skip, default)]`.
//! - **Contradictory attributes:** Explains why `from` + `skip` can't be combined.
//! - **Non-struct usage:** Explains that only named-field structs are supported.
//!
//! ---
//!
//! ## How It Works
//!
//! The `#[derive(MapFrom)]` macro:
//!
//! 1. **Parses** the `#[map_from(Source)]` attribute to find the source type.
//! 2. **Inspects** each field's `#[map(...)]` attributes.
//! 3. **Generates** an `impl From<Source> for Target` block with the correct
//!    field assignments — direct copies, renames, `.into()` calls, or function applications.
//!
//! All work happens at **compile time**. The generated code is identical to what you
//! would write by hand — zero runtime overhead, zero allocations, zero dependencies.
//!
//! ---
//!
//! ## Limitations
//!
//! **v0.2 constraints:**
//!
//! - Only named-field structs. Tuple structs and enums are not yet supported.
//! - Generic target structs work; generic source types need manual handling.

#![deny(missing_docs)]

/// Derive macro that generates `impl From<Source> for Target` by mapping struct fields.
///
/// Place `#[derive(MapFrom)]` on your target struct along with `#[map_from(SourceType)]`
/// to auto-generate the `From` implementation.
///
/// # Example
///
/// ```rust
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
/// let entity = UserEntity {
///     name: "Khushi".to_string(),
///     email: "khushi@gmail.com".to_string(),
/// };
/// let response: UserResponse = entity.into();
/// assert_eq!(response.name, "Khushi");
/// ```
///
/// # Field Attributes
///
/// | Attribute | Description |
/// |:----------|:------------|
/// | `#[map(from = "name")]` | Map from a differently-named source field |
/// | `#[map(skip, default)]` | Skip this field, use `Default::default()` |
/// | `#[map(into)]` | Call `.into()` on the source value |
/// | `#[map(with = "fn_path")]` | Apply a custom conversion function |
///
/// Attributes can be combined: `#[map(from = "old_name", with = "convert_fn")]`
pub use struct_mapper_derive::MapFrom;

/// Derive macro that generates `impl TryFrom<Source> for Target` by mapping struct fields.
///
/// Use this when one or more field conversions can fail. The generated implementation
/// returns `Result<Self, MapError>`.
///
/// # Example
///
/// ```rust
/// use struct_mapper::TryMapFrom;
/// use std::convert::TryInto;
///
/// struct RawInput {
///     count: i64,
///     name: String,
/// }
///
/// #[derive(TryMapFrom)]
/// #[try_map_from(RawInput)]
/// struct ValidInput {
///     #[map(try_into)]
///     count: u32,
///     name: String,
/// }
///
/// let raw = RawInput { count: 42, name: "Alice".into() };
/// let valid: ValidInput = raw.try_into().unwrap();
/// assert_eq!(valid.count, 42);
/// assert_eq!(valid.name, "Alice");
/// ```
///
/// # Field Attributes
///
/// All `MapFrom` attributes work here, plus:
///
/// | Attribute | Description |
/// |:----------|:------------|
/// | `#[map(try_into)]` | Call `.try_into()` on the source value (fallible) |
/// | `#[map(try_with = "fn")]` | Apply a fallible conversion function |
pub use struct_mapper_derive::TryMapFrom;

/// Error type for fallible struct mapping via [`TryMapFrom`].
///
/// Contains the field name where the conversion failed and the underlying error.
///
/// # Example
///
/// ```rust
/// use struct_mapper::{TryMapFrom, MapError};
/// use std::convert::TryInto;
///
/// struct Source { value: i64 }
///
/// #[derive(Debug, TryMapFrom)]
/// #[try_map_from(Source)]
/// struct Target {
///     #[map(try_into)]
///     value: u32,
/// }
///
/// let src = Source { value: -1 };
/// let err = Target::try_from(src).unwrap_err();
/// assert_eq!(err.field, "value");
/// ```
#[derive(Debug)]
pub struct MapError {
    /// The name of the field where conversion failed.
    pub field: &'static str,
    /// The underlying error that caused the failure.
    pub source: Box<dyn std::error::Error + Send + Sync>,
}

impl MapError {
    /// Create a new mapping error for a specific field.
    pub fn field<E: std::error::Error + Send + Sync + 'static>(
        field: &'static str,
        error: E,
    ) -> Self {
        MapError {
            field,
            source: Box::new(error),
        }
    }
}

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mapping failed at field `{}`: {}",
            self.field, self.source
        )
    }
}

impl std::error::Error for MapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}
