use struct_mapper::TryMapFrom;

struct Source {
    name: String,
    age: u32,
}

#[derive(Debug, TryMapFrom)]
#[try_map_from(Source)]
struct Target {
    name: String,
    age: u32,
}

#[test]
fn try_from_basic_all_infallible() {
    let src = Source {
        name: "Deendayal".into(),
        age: 25,
    };
    let target: Target = src.try_into().unwrap();
    assert_eq!(target.name, "Deendayal");
    assert_eq!(target.age, 25);
}
