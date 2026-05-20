# Attribute Reference

This page provides a quick summary of all attributes available in `struct-mapper`.

## Struct-Level Attributes

| Attribute | Required | Description |
|:----------|:--------:|:------------|
| `#[map_from(Type)]` | **Yes** | Specifies the source type for `From<Type>` generation |

## Field-Level Attributes

| Attribute | Description |
|:----------|:------------|
| `#[map(from = "name")]` | Map from a differently-named source field |
| `#[map(skip, default)]` | Skip this field; use `Default::default()` |
| `#[map(into)]` | Call `.into()` on the source field value |
| `#[map(with = "path")]` | Apply a conversion function `fn(SourceFieldType) -> TargetFieldType` |

Attributes can be combined, for example: `#[map(from = "old", with = "convert")]`
