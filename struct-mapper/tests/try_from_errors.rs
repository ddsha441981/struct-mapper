use struct_mapper::TryMapFrom;

struct Source {
    value: i64,
}

#[derive(Debug, TryMapFrom)]
#[try_map_from(Source)]
#[allow(dead_code)]
struct Target {
    #[map(try_into)]
    value: u32,
}

#[test]
fn error_contains_field_name() {
    let err = Target::try_from(Source { value: -1 }).unwrap_err();
    assert_eq!(err.field, "value");
}

#[test]
fn error_display_is_readable() {
    let err = Target::try_from(Source { value: -1 }).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("value"), "should mention field name");
    assert!(msg.contains("mapping failed"), "should have clear prefix");
}

#[test]
fn error_source_chain_works() {
    use std::error::Error;
    let err = Target::try_from(Source { value: -1 }).unwrap_err();
    assert!(
        err.source().is_some(),
        "should have underlying source error"
    );
}
