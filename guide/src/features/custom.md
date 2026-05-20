# Custom Functions

For complex mappings that cannot be solved by simple type conversions, you can provide a custom transformation function using `#[map(with = "path::to::function")]`.

The provided function must take the source field type and return the target field type.

```rust
use struct_mapper::MapFrom;

// Custom conversion logic
fn cents_to_dollars(cents: u64) -> f64 {
    cents as f64 / 100.0
}

struct PriceEntity { amount_cents: u64 }

#[derive(MapFrom)]
#[map_from(PriceEntity)]
struct PriceDTO {
    // Passes `amount_cents` to `cents_to_dollars`
    #[map(from = "amount_cents", with = "cents_to_dollars")]
    amount: f64,
}

fn main() {
    let dto: PriceDTO = PriceEntity { amount_cents: 1999 }.into();
    
    assert!((dto.amount - 19.99).abs() < f64::EPSILON);
}
```
