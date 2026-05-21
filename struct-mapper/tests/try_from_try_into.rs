use struct_mapper::TryMapFrom;

struct Source {
    count: i64,
    name: String,
}

#[derive(Debug, TryMapFrom)]
#[try_map_from(Source)]
struct Target {
    #[map(try_into)]
    count: u32,
    name: String,
}

#[test]
fn try_into_success() {
    let src = Source {
        count: 42,
        name: "Alice".into(),
    };
    let target: Target = src.try_into().unwrap();
    assert_eq!(target.count, 42);
    assert_eq!(target.name, "Alice");
}

#[test]
fn try_into_failure_negative() {
    let src = Source {
        count: -1,
        name: "Bob".into(),
    };
    let result = Target::try_from(src);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.field, "count");
}

#[test]
fn try_into_failure_overflow() {
    let src = Source {
        count: i64::from(u32::MAX) + 1,
        name: "Charlie".into(),
    };
    let result = Target::try_from(src);
    assert!(result.is_err());
}
