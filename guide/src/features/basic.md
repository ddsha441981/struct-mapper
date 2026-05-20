# Basic Mapping

When source and target fields have the **same name** and the **same type**, you don't need any special annotations. The macro maps them automatically.

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

fn main() {
    let t: Target = Source { name: "Bob".into(), age: 25 }.into();
    assert_eq!(t.name, "Bob");
    assert_eq!(t.age, 25);
}
```
