# Introduction

Welcome to the **`struct-mapper`** User Guide!

`struct-mapper` provides a powerful derive macro to auto-generate `impl From<Source> for Target` by mapping struct fields. This eliminates the tedious boilerplate typically associated with struct-to-struct conversions in Rust backend applications.

## Why use struct-mapper?

Every Rust backend developer writes code like this dozens of times:

```rust
impl From<UserEntity> for UserResponse {
    fn from(e: UserEntity) -> Self {
        UserResponse {
            name: e.name,
            email: e.email,
            age: e.age,
            display_name: e.first_name,            // renamed
            address: e.address.into(),              // nested
            created_at: Default::default(),         // not in source
        }
    }
}
```

With `struct-mapper`, you replace all of that with a single derive macro:

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

### Benefits

- **Zero Boilerplate:** Focus on your business logic, not copying fields.
- **Zero Runtime Cost:** The macro generates the exact same `From` block you would write by hand. No reflection, no runtime overhead.
- **Clear Error Messages:** Built-in safeguards ensure you get perfect, span-accurate compile errors if you make a mistake.
