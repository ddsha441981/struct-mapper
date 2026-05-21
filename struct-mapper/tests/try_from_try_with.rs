use std::num::ParseIntError;
use struct_mapper::TryMapFrom;

fn parse_port(s: String) -> Result<u16, ParseIntError> {
    s.parse::<u16>()
}

struct Source {
    port: String,
    host: String,
}

#[derive(Debug, TryMapFrom)]
#[try_map_from(Source)]
struct Target {
    #[map(try_with = "parse_port")]
    port: u16,
    host: String,
}

#[test]
fn try_with_success() {
    let src = Source {
        port: "8080".into(),
        host: "localhost".into(),
    };
    let target: Target = src.try_into().unwrap();
    assert_eq!(target.port, 8080);
    assert_eq!(target.host, "localhost");
}

#[test]
fn try_with_failure_invalid_port() {
    let src = Source {
        port: "not_a_number".into(),
        host: "localhost".into(),
    };
    let result = Target::try_from(src);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().field, "port");
}

#[test]
fn try_with_failure_port_overflow() {
    let src = Source {
        port: "99999".into(),
        host: "localhost".into(),
    };
    let result = Target::try_from(src);
    assert!(result.is_err());
}
