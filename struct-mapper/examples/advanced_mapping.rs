//! Advanced mapping example — renamed fields, skip, into, and custom functions.

use struct_mapper::MapFrom;

//  Source structs

struct AddressEntity {
    street: String,
    city: String,
}

struct OrderEntity {
    order_id: u64,
    user_name: String,
    total_cents: u64,
    address: AddressEntity,
}

//  Target structs

#[derive(Debug)]
#[allow(dead_code)]
struct AddressResponse {
    street: String,
    city: String,
}

impl From<AddressEntity> for AddressResponse {
    fn from(e: AddressEntity) -> Self {
        AddressResponse {
            street: e.street,
            city: e.city,
        }
    }
}

fn cents_to_dollars(cents: u64) -> f64 {
    cents as f64 / 100.0
}

#[derive(Debug, MapFrom)]
#[map_from(OrderEntity)]
struct OrderResponse {
    // Same name: direct mapping
    order_id: u64,

    // Renamed: source field is "user_name", target is "name"
    #[map(from = "user_name")]
    name: String,

    // Custom function: convert cents to dollars
    #[map(with = "cents_to_dollars", from = "total_cents")]
    total_dollars: f64,

    // Into: calls AddressEntity.into() → AddressResponse
    #[map(into)]
    address: AddressResponse,

    // Skipped: not in source, uses Default
    #[map(skip, default)]
    request_id: String,
}

fn main() {
    let order = OrderEntity {
        order_id: 42,
        user_name: "Deendayal".to_string(),
        total_cents: 1999,
        address: AddressEntity {
            street: "123 Jaipur".to_string(),
            city: "Springfield".to_string(),
        },
    };

    let response: OrderResponse = order.into();

    println!("{:#?}", response);
    assert_eq!(response.order_id, 42);
    assert_eq!(response.name, "Deendayal");
    assert!((response.total_dollars - 19.99).abs() < f64::EPSILON);
    assert_eq!(response.address.city, "Springfield");
    assert_eq!(response.request_id, ""); // Default::default()

    println!("✅ Advanced mapping works!");
}
