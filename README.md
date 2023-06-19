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

let user = User {
    id: 1,
    name: "Alice".to_string(),
};

user.id_eq(1)
    .name_eq("Alice".to_string());

user.id_ne(2)
    .name_ne("Bob".to_string());
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

let point = Point { x: 1, y: 2 };

point.x_eq(1).y_eq(2);

point.x_ne(9).y_ne(9);
```
