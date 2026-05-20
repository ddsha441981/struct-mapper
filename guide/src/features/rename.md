# Field Renaming

When the target field name differs from the source field name, use the `#[map(from = "...")]` attribute to tell the macro which source field to read.

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

fn main() {
    let row = DbRow { user_name: "Charlie".into(), user_age: 35 };
    let user: ApiUser = row.into();
    
    assert_eq!(user.name, "Charlie");
    assert_eq!(user.age, 35);
}
```
