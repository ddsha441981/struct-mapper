use std::num::ParseIntError;
use struct_mapper::TryMapFrom;

fn to_upper(s: String) -> String {
    s.to_uppercase()
}

fn parse_age(s: String) -> Result<u8, ParseIntError> {
    s.parse::<u8>()
}

#[allow(dead_code)]
struct Source {
    id: i64,
    raw_name: String,
    age_str: String,
    extra: String,
}

#[derive(Debug, TryMapFrom)]
#[try_map_from(Source)]
struct Target {
    #[map(try_into)]
    id: u32, // fallible: i64 -> u32

    #[map(from = "raw_name", with = "to_upper")]
    name: String, // infallible rename + transform

    #[map(from = "age_str", try_with = "parse_age")]
    age: u8, // fallible rename + transform

    #[map(skip, default)]
    request_id: String, // skipped with default
}

#[test]
fn mixed_all_success() {
    let src = Source {
        id: 100,
        raw_name: "deendayal".into(),
        age_str: "25".into(),
        extra: "ignored".into(),
    };
    let target: Target = src.try_into().unwrap();
    assert_eq!(target.id, 100);
    assert_eq!(target.name, "DEENDAYAL");
    assert_eq!(target.age, 25);
    assert_eq!(target.request_id, "");
}

#[test]
fn mixed_fail_on_id() {
    let src = Source {
        id: -5,
        raw_name: "test".into(),
        age_str: "25".into(),
        extra: "x".into(),
    };
    let err = Target::try_from(src).unwrap_err();
    assert_eq!(err.field, "id");
}

#[test]
fn mixed_fail_on_age() {
    let src = Source {
        id: 1,
        raw_name: "test".into(),
        age_str: "300".into(), // > 255, won't fit in u8
        extra: "x".into(),
    };
    let err = Target::try_from(src).unwrap_err();
    assert_eq!(err.field, "age");
}
