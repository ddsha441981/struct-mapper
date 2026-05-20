//! Basic mapping example — same field names, same types.

use struct_mapper::MapFrom;

// Source: imagine this comes from your database layer
struct UserEntity {
    name: String,
    email: String,
    age: u32,
}

// Target: your API response — auto-generates From<UserEntity>
#[derive(Debug, MapFrom)]
#[map_from(UserEntity)]
struct UserResponse {
    name: String,
    email: String,
    age: u32,
}

fn main() {
    let entity = UserEntity {
        name: "Khushi".to_string(),
        email: "khushi@gmail.com".to_string(),
        age: 30,
    };

    // Zero-boilerplate conversion!
    let response: UserResponse = entity.into();

    println!("{:?}", response);
    assert_eq!(response.name, "Khushi");
    assert_eq!(response.email, "khushi@gmail.com");
    assert_eq!(response.age, 30);

    println!("✅ Basic mapping works!");
}
