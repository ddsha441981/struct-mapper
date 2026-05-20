<div align="center">

# 🔄 struct-mapper

**Derive macro to auto-generate `From<Source>` for your structs**
**— zero boilerplate field mapping.**

[![CI](https://github.com/ddsha441981/struct-mapper/actions/workflows/ci.yml/badge.svg)](https://github.com/ddsha441981/struct-mapper/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange?style=flat-square&logo=rust)](https://crates.io/crates/struct-mapper)
[![Docs](https://img.shields.io/badge/docs.rs-struct--mapper-blue?style=flat-square&logo=docs.rs)](https://docs.rs/struct-mapper)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green?style=flat-square)](https://github.com/ddsha441981/struct-mapper)
[![MSRV](https://img.shields.io/badge/MSRV-1.70.0-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)

[📖 Documentation](https://docs.rs/struct-mapper) · [📦 Crates.io](https://crates.io/crates/struct-mapper) · [🐛 Report Bug](https://github.com/ddsha441981/struct-mapper/issues) · [💡 Request Feature](https://github.com/ddsha441981/struct-mapper/issues)

</div>

---

Stop writing tedious manual `From` implementations for struct-to-struct conversions. `struct-mapper` generates them at **compile time** with **zero runtime overhead**.

```rust
use struct_mapper::MapFrom;

struct UserEntity {
    name: String,
    email: String,
    age: u32,
}

#[derive(MapFrom)]
#[map_from(UserEntity)]
struct UserResponse {
    name: String,
    email: String,
    age: u32,
}

// That's it! Now you can do:
let entity = UserEntity { name: "Alice".into(), email: "a@b.com".into(), age: 30 };
let response: UserResponse = entity.into();
```

> No runtime cost. No reflection. Just a clean `impl From<>` generated at compile time.

---

## ✨ Why struct-mapper?

Every Rust backend developer writes this **dozens of times**:

<table>
<tr>
<th>😩 Before — Manual boilerplate</th>
<th>🚀 After — One derive</th>
</tr>
<tr>
<td>

```rust
impl From<UserEntity> for UserResponse {
    fn from(e: UserEntity) -> Self {
        UserResponse {
            name: e.name,
            email: e.email,
            age: e.age,
            display_name: e.first_name,
            address: e.address.into(),
            created_at: Default::default(),
        }
    }
}
```

</td>
<td>

```rust
#[derive(MapFrom)]
#[map_from(UserEntity)]
struct UserResponse {
    name: String,
    email: String,
    age: u32,
    #[map(from = "first_name")]
    display_name: String,
    #[map(into)]
    address: AddressResponse,
    #[map(skip, default)]
    created_at: String,
}
```

</td>
</tr>
</table>

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
struct-mapper = "0.1"
```

**Minimum Supported Rust Version:** `1.70.0`

---

## 🚀 Features

### 1️⃣ Basic Mapping — Same Name, Same Type

Fields with matching names are mapped automatically. No attributes needed.

```rust
use struct_mapper::MapFrom;

struct Source {
    name: String,
    age: u32,
}

#[derive(MapFrom)]
#[map_from(Source)]
struct Target {
    name: String,
    age: u32,
}

let target: Target = Source { name: "Alice".into(), age: 30 }.into();
assert_eq!(target.name, "Alice");
```

### 2️⃣ Renamed Fields — `#[map(from = "...")]`

When source and target field names differ:

```rust
use struct_mapper::MapFrom;

struct DbRow {
    user_name: String,
    user_age: u32,
}

#[derive(MapFrom)]
#[map_from(DbRow)]
struct ApiUser {
    #[map(from = "user_name")]
    name: String,
    #[map(from = "user_age")]
    age: u32,
}
```

### 3️⃣ Skip + Default — `#[map(skip, default)]`

For fields that don't exist in the source struct:

```rust
use struct_mapper::MapFrom;

struct Entity {
    name: String,
}

#[derive(MapFrom)]
#[map_from(Entity)]
struct Response {
    name: String,
    #[map(skip, default)]
    request_id: String,    // → Default::default() = ""
    #[map(skip, default)]
    retry_count: u32,      // → Default::default() = 0
}
```

### 4️⃣ Nested Conversion — `#[map(into)]`

For fields where the source type implements `Into<TargetType>`:

```rust
use struct_mapper::MapFrom;

struct AddressEntity { street: String, city: String }

#[derive(MapFrom)]
#[map_from(AddressEntity)]
struct AddressDTO { street: String, city: String }

struct OrderEntity {
    id: u64,
    address: AddressEntity,
}

#[derive(MapFrom)]
#[map_from(OrderEntity)]
struct OrderDTO {
    id: u64,
    #[map(into)]
    address: AddressDTO,   // → source.address.into()
}
```

### 5️⃣ Custom Function — `#[map(with = "...")]`

For complex transformations using any function:

```rust
use struct_mapper::MapFrom;

fn cents_to_dollars(cents: u64) -> f64 {
    cents as f64 / 100.0
}

struct PriceEntity {
    amount_cents: u64,
}

#[derive(MapFrom)]
#[map_from(PriceEntity)]
struct PriceResponse {
    #[map(from = "amount_cents", with = "cents_to_dollars")]
    amount: f64,
}
```

### 🔗 Combine Everything

All attributes work together seamlessly:

```rust
use struct_mapper::MapFrom;

struct OrderEntity {
    order_id: u64,
    user_name: String,
    total_cents: u64,
    address: AddressEntity,
}

#[derive(MapFrom)]
#[map_from(OrderEntity)]
struct OrderResponse {
    order_id: u64,                                      // direct
    #[map(from = "user_name")]
    name: String,                                       // renamed
    #[map(from = "total_cents", with = "cents_to_dollars")]
    total: f64,                                         // renamed + custom fn
    #[map(into)]
    address: AddressDTO,                                // nested conversion
    #[map(skip, default)]
    request_id: String,                                 // skipped
}
```

---

## 📋 Attribute Reference

| Attribute | Applies To | Description |
|:----------|:----------:|:------------|
| `#[map_from(Type)]` | Struct | Source type to generate `From<Type>` for |
| `#[map(from = "name")]` | Field | Map from a differently-named source field |
| `#[map(skip, default)]` | Field | Skip this field, use `Default::default()` |
| `#[map(into)]` | Field | Call `.into()` on the source value |
| `#[map(with = "fn")]` | Field | Apply a custom conversion function |

> 💡 **Tip:** Attributes can be combined: `#[map(from = "old_name", with = "convert_fn")]`

---

## 🛡️ Error Messages

`struct-mapper` provides **clear, actionable error messages** that tell you exactly what went wrong and how to fix it:

```
error: missing `#[map_from(SourceType)]` attribute.
       Add `#[map_from(YourSourceStruct)]` to specify which struct to map from.

       Example:
         #[derive(MapFrom)]
         #[map_from(UserEntity)]
         struct UserResponse { ... }
```

```
error: `#[map(skip)]` requires `#[map(skip, default)]`.
       When skipping a field, you must provide a default value.

       Fix: #[map(skip, default)]
```

---

## 📊 Comparison

How does `struct-mapper` compare to alternatives?

| Feature | **struct-mapper** | derive-into | more-convert | structural-convert |
|:--------|:-:|:-:|:-:|:-:|
| Same-field mapping | ✅ | ✅ | ✅ | ✅ |
| Field renaming | ✅ | ✅ | ✅ | ⚠️ |
| Skip + default | ✅ | ⚠️ | ⚠️ | ⚠️ |
| Nested `.into()` | ✅ | ✅ | ❌ | ⚠️ |
| Custom function | ✅ | ✅ | ⚠️ | ⚠️ |
| **Clear error messages** | ✅ | ❌ | ❌ | ❌ |
| **Clean syntax** | ✅ | ⚠️ | ⚠️ | ⚠️ |
| Compile-time only | ✅ | ✅ | ✅ | ✅ |
| Zero runtime deps | ✅ | ✅ | ✅ | ✅ |

---

## 🗺️ Roadmap

- [x] `From` — infallible struct conversion
- [x] Field renaming, skipping, nesting, custom functions
- [x] Clear compile-time error messages
- [ ] `TryFrom` — fallible conversions (`v0.2`)
- [ ] Enum variant mapping (`v0.3`)
- [ ] Bi-directional mapping (`v0.4`)

---

## ⚠️ Limitations (v0.1)

- Only `From` (infallible conversion). `TryFrom` is planned for v0.2.
- Only named struct fields. Tuple structs and enums are not yet supported.
- Generics on the target struct are supported; generic source types require manual annotation.

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 👨‍💻 Author

<table>
<tr>
<td>

**Deendayal Kumawat**

[![LinkedIn](https://img.shields.io/badge/LinkedIn-0077B5?style=flat-square&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/deendayal-kumawat/)
[![GitHub](https://img.shields.io/badge/GitHub-181717?style=flat-square&logo=github&logoColor=white)](https://github.com/ddsha441981)
[![Email](https://img.shields.io/badge/Email-0078D4?style=flat-square&logo=microsoft-outlook&logoColor=white)](mailto:deendayal_kumawat@outlook.com)

</td>
</tr>
</table>

---

## 📄 License

Licensed under either of:

- **Apache License, Version 2.0** — [LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>
- **MIT License** — [LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>

at your option.

---

<div align="center">

**⭐ If you find this useful, please consider giving it a star! ⭐**

*Made with ❤️ in Rust*

</div>
