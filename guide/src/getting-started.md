# Getting Started

Adding `struct-mapper` to your project is straightforward.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
struct-mapper = "0.1"
```

*Minimum Supported Rust Version: `1.70.0`*

## Basic Example

Here is a simple example to get you started. Import the `MapFrom` trait and apply it to your target struct. Use `#[map_from(SourceStruct)]` to specify the source.

```rust
use struct_mapper::MapFrom;

// 1. Define your source struct
struct UserEntity {
    name: String,
    email: String,
    age: u32,
}

// 2. Define your target struct and derive MapFrom
#[derive(Debug, MapFrom)]
#[map_from(UserEntity)]
struct UserResponse {
    name: String,
    email: String,
    age: u32,
}

fn main() {
    let entity = UserEntity {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
    };

    // 3. The Into implementation is now automatically available!
    let response: UserResponse = entity.into();
    
    println!("{:?}", response);
}
```
