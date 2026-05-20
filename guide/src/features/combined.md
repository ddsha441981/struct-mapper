# Combining Attributes

Attributes in `struct-mapper` are designed to be composable. You can combine multiple instructions into a single `#[map(...)]` attribute to define complex behaviors.

```rust
use struct_mapper::MapFrom;

fn to_upper(s: String) -> String { s.to_uppercase() }

struct Source {
    id: u64,
    user_name: String,
    raw_email: String,
}

#[derive(Debug, MapFrom)]
#[map_from(Source)]
struct Target {
    id: u64,                                       // direct
    
    #[map(from = "user_name")]
    name: String,                                  // renamed
    
    // Combining `from` and `with`
    #[map(from = "raw_email", with = "to_upper")]
    email: String,                                 
    
    // Combining `skip` and `default`
    #[map(skip, default)]
    request_id: String,                            // skipped
}

fn main() {
    let t: Target = Source {
        id: 1,
        user_name: "Eve".into(),
        raw_email: "eve@test.com".into(),
    }.into();

    assert_eq!(t.id, 1);
    assert_eq!(t.name, "Eve");
    assert_eq!(t.email, "EVE@TEST.COM");
    assert_eq!(t.request_id, "");
}
```
