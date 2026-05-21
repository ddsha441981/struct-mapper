# Attribute Reference

This page provides a quick summary of all attributes available in `struct-mapper`.

## Struct-Level Attributes

| Attribute | Required | Description |
|:----------|:--------:|:------------|
| `#[map_from(Type)]` | **Yes** (with `MapFrom`) | Specifies the source type for `From<Type>` generation |
| `#[try_map_from(Type)]` | **Yes** (with `TryMapFrom`) | Specifies the source type for `TryFrom<Type>` generation |

## Field-Level Attributes

| Attribute | Description |
|:----------|:------------|
| `#[map(from = "name")]` | Map from a differently-named source field |
| `#[map(skip, default)]` | Skip this field; use `Default::default()` |
| `#[map(into)]` | Call `.into()` on the source field value |
| `#[map(with = "path")]` | Apply a conversion function `fn(SourceFieldType) -> TargetFieldType` |
| `#[map(try_into)]` | Call `.try_into()` on the source field value *(TryMapFrom only)* |
| `#[map(try_with = "path")]` | Apply a fallible function `fn(S) -> Result<T, E>` *(TryMapFrom only)* |

Attributes can be combined, for example: `#[map(from = "old", try_with = "parse")]`
