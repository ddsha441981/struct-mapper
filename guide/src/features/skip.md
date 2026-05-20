# Skip & Default

Sometimes your target struct contains fields that are not present in the source struct (e.g., IDs generated upon insertion, timestamps, or request-specific context). 

Use `#[map(skip, default)]` to ignore the source entirely for a specific field and populate it using `Default::default()`.

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
    request_id: String,    // Populated with String::default() -> ""
    
    #[map(skip, default)]
    retry_count: u32,      // Populated with u32::default() -> 0
}

fn main() {
    let r: Response = Entity { name: "Dave".into() }.into();
    
    assert_eq!(r.name, "Dave");
    assert_eq!(r.request_id, "");
    assert_eq!(r.retry_count, 0);
}
```

> **Note:** The `skip` attribute **requires** `default` to be explicitly provided (`#[map(skip, default)]`). Using `#[map(skip)]` alone will result in a compile error, as `struct-mapper` enforces explicitness to avoid silent bugs.
