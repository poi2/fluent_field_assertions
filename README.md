# FluentFieldAssertions

FluentFieldAssertions is a procedural macro for generating fluent assertions on fields.

## Example

This is basic case.

```rust
use fluent_field_assertions::FluentFieldAssertions;

#[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
struct User {
    id: usize,
    name: String,
}

User {
    id: 1,
    name: "Alice".to_string(),
}
.id_eq(1)
.name_eq("Alice".to_string());
```

You can also use in generic struct.

```rust
use core::fmt::Debug;
use fluent_field_assertions::FluentFieldAssertions;

#[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
struct Point<T>
where
    T: Eq + Debug,
{
    x: T,
    y: T,
}

Point { x: 1, y: 2 }.x_eq(1).y_eq(2);
```
