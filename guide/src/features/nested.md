# Nested Conversions

It is incredibly common to have structs composed of other structs. When mapping from a parent source struct to a parent target struct, you usually want to map the children as well.

If the inner source type implements `Into<TargetFieldType>` (which might be because you derived `MapFrom` on the child type!), you can use `#[map(into)]` to automatically invoke `.into()`.

```rust
use struct_mapper::MapFrom;

// --- Inner structs ---
struct AddressEntity { city: String }

#[derive(Debug, PartialEq, MapFrom)]
#[map_from(AddressEntity)]
struct AddressDTO { city: String }

// --- Outer structs ---
struct OrderEntity {
    id: u64,
    address: AddressEntity,
}

#[derive(Debug, MapFrom)]
#[map_from(OrderEntity)]
struct OrderDTO {
    id: u64,
    
    // Automatically calls `source.address.into()`
    #[map(into)]
    address: AddressDTO,  
}

fn main() {
    let order = OrderEntity {
        id: 42,
        address: AddressEntity { city: "Springfield".into() },
    };
    
    let dto: OrderDTO = order.into();
    
    assert_eq!(dto.id, 42);
    assert_eq!(dto.address.city, "Springfield");
}
```
