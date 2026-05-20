# Error Messages

`struct-mapper` is built to provide a premium developer experience. A major part of this is providing excellent, span-accurate compile errors when something goes wrong.

Instead of vague syntax errors, `struct-mapper` attempts to identify your intent and guide you to the right solution.

### Examples

**Missing `#[map_from]`:**
If you derive `MapFrom` but forget to specify the source struct, you will see exactly what to add and where:

```text
error: missing `#[map_from(SourceType)]` attribute.
       Add `#[map_from(YourSourceStruct)]` to specify which struct to map from.

       Example:
         #[derive(MapFrom)]
         #[map_from(UserEntity)]
         struct UserResponse { ... }
```

**`#[map(skip)]` without `default`:**
`struct-mapper` forces you to be explicit to avoid accidental behavior. If you want to skip a field, you must explicitly tell it to use a default:

```text
error: `#[map(skip)]` requires `#[map(skip, default)]`.
       When skipping a field, you must provide a default value.

       Fix: #[map(skip, default)]
```

**Contradictory Attributes:**
If you try to map *from* a field while also telling the macro to *skip* it, `struct-mapper` catches the contradiction:

```text
error: `skip` and `from` cannot be used together.
```
