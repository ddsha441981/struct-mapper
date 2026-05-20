use struct_mapper::MapFrom;

//  Test 1: Basic same-field mapping

struct Source1 {
    name: String,
    age: u32,
}

#[derive(Debug, PartialEq, MapFrom)]
#[map_from(Source1)]
struct Target1 {
    name: String,
    age: u32,
}

#[test]
fn test_basic_same_fields() {
    let source = Source1 {
        name: "Alice".to_string(),
        age: 30,
    };
    let target: Target1 = source.into();
    assert_eq!(target.name, "Alice");
    assert_eq!(target.age, 30);
}

//  Test 2: Renamed field

struct Source2 {
    user_name: String,
    user_age: u32,
}

#[derive(Debug, PartialEq, MapFrom)]
#[map_from(Source2)]
struct Target2 {
    #[map(from = "user_name")]
    name: String,
    #[map(from = "user_age")]
    age: u32,
}

#[test]
fn test_renamed_fields() {
    let source = Source2 {
        user_name: "Bob".to_string(),
        user_age: 25,
    };
    let target: Target2 = source.into();
    assert_eq!(target.name, "Bob");
    assert_eq!(target.age, 25);
}

//  Test 3: Skip + Default

struct Source3 {
    name: String,
}

#[derive(Debug, PartialEq, MapFrom)]
#[map_from(Source3)]
struct Target3 {
    name: String,
    #[map(skip, default)]
    extra: String,
    #[map(skip, default)]
    count: u32,
}

#[test]
fn test_skip_default() {
    let source = Source3 {
        name: "Charlie".to_string(),
    };
    let target: Target3 = source.into();
    assert_eq!(target.name, "Charlie");
    assert_eq!(target.extra, ""); // String::default()
    assert_eq!(target.count, 0); // u32::default()
}

//  Test 4: Into conversion

struct Inner {
    value: String,
}

#[derive(Debug, PartialEq)]
struct InnerDTO {
    value: String,
}

impl From<Inner> for InnerDTO {
    fn from(i: Inner) -> Self {
        InnerDTO { value: i.value }
    }
}

struct Source4 {
    name: String,
    inner: Inner,
}

#[derive(Debug, PartialEq, MapFrom)]
#[map_from(Source4)]
struct Target4 {
    name: String,
    #[map(into)]
    inner: InnerDTO,
}

#[test]
fn test_into_conversion() {
    let source = Source4 {
        name: "Dave".to_string(),
        inner: Inner {
            value: "nested".to_string(),
        },
    };
    let target: Target4 = source.into();
    assert_eq!(target.name, "Dave");
    assert_eq!(target.inner.value, "nested");
}

//  Test 5: Custom function

fn double(x: u32) -> u64 {
    (x * 2) as u64
}

struct Source5 {
    name: String,
    count: u32,
}

#[derive(Debug, PartialEq, MapFrom)]
#[map_from(Source5)]
struct Target5 {
    name: String,
    #[map(with = "double")]
    count: u64,
}

#[test]
fn test_custom_function() {
    let source = Source5 {
        name: "Eve".to_string(),
        count: 21,
    };
    let target: Target5 = source.into();
    assert_eq!(target.name, "Eve");
    assert_eq!(target.count, 42);
}

//  Test 6: Combined attributes

fn to_upper(s: String) -> String {
    s.to_uppercase()
}

struct Source6 {
    first_name: String,
    raw_email: String,
    age: u32,
}

#[derive(Debug, PartialEq, MapFrom)]
#[map_from(Source6)]
struct Target6 {
    #[map(from = "first_name")]
    name: String,
    #[map(from = "raw_email", with = "to_upper")]
    email: String,
    age: u32,
    #[map(skip, default)]
    id: u64,
}

#[test]
fn test_combined_attributes() {
    let source = Source6 {
        first_name: "Frank".to_string(),
        raw_email: "frank@test.com".to_string(),
        age: 40,
    };
    let target: Target6 = source.into();
    assert_eq!(target.name, "Frank");
    assert_eq!(target.email, "FRANK@TEST.COM");
    assert_eq!(target.age, 40);
    assert_eq!(target.id, 0);
}
