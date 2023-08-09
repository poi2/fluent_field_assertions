# FluentFieldAssertions

FluentFieldAssertions is a library that allows you to write tests in a natural language-like syntax.  
With this library, you can perform field assertions in an intuitive and readable way.

## Features

- Natural Language Syntax: You can write test code in a syntax that closely resembles natural language.
- Method Chaining: You can chain assertions as method calls, allowing you to express multiple assertions in a single statement.
- Readability and Maintainability: The code becomes simple and readable, improving the maintainability of your tests.

## Uses

FluentFieldAssertions generated the following methods for each field.

- `fn {field}_eq(&self, expected: {field_type}) -> &Self`
  - Asserts that the field is equal to the expected value.
- `fn {field}_ne(&self, expected: {field_type}) -> &Self`
  - Asserts that the field is not equal to the expected value.
- `fn {field}_satisfies(&self, pred: impl FnOnce(&{field_type}) -> bool) -> &Self`
  - Asserts that the field satisfies the predicate.

```rust
use fluent_field_assertions::FluentFieldAssertions;

#[derive(FluentFieldAssertions)]
struct User {
    id: usize,
    name: String,
    age: usize,
}

let user = User {
    id: 1,
    name: "Alice".to_string(),
    age: 17,
};

// You can write tests in a natural language-like syntax.
user.id_eq(&1)
    .name_eq(&"Alice".to_string())
    .age_satisfies(|age| age < &18);

// Same as above.
assert_eq!(user.id, 1);
assert_eq!(user.name, "Alice".to_string());
assert!(user.age < 18);
```

## Examples

### Basic struct

You can use it in the basic struct.  
Each field must implement the traits `Eq` and `Debug`.

```rust
use fluent_field_assertions::FluentFieldAssertions;

#[derive(FluentFieldAssertions)]
struct User {
    id: usize,
    name: String,
    age: usize,
    // You can skip assertion_methods for some fields.
    #[assertions(skip)]
    score: f64,
}

let user = User {
    id: 1,
    name: "Alice".to_string(),
    age: 17,
    score: 95.0,
};

// You can use `{field}_eq` to assert_eq!.
user.name_eq(&"Alice".to_string());
assert_eq!(user.name, "Alice".to_string()); // Same as above.

// You can use `{field}_ne` to assert_ne!.
user.name_ne(&"Bob".to_string());
assert_ne!(user.name, "Bob".to_string()); // Same as above.

// You can use `{field}_satisfies` to assert!.
user.name_satisfies(|name| name.starts_with('A'));
assert!(user.name.starts_with('A')); // Same as above.

// You can chain assertions as method calls.
user.id_eq(&1)
    .name_eq(&"Alice".to_string())
    .age_satisfies(|age| age < &18);

// Same as above.
assert_eq!(user.id, 1);
assert_eq!(user.name, "Alice".to_string());
assert!(user.age < 18);
```

### Generic struct

You can also use it in the generic struct.  
In that case, Generics type `T` must implement the traits `Eq` and `Debug`.

```rust
use core::fmt::Debug;
use fluent_field_assertions::FluentFieldAssertions;

#[derive(FluentFieldAssertions)]
struct Point<T>
// Generics type `T` must implement trait `Eq` and `Debug`.
where
    T: Eq + Debug,
{
    x: T,
    y: T,
    z: T,
    // You can skip assertion_methods for some fields.
    #[assertions(skip)]
    #[allow(dead_code)]
    t: T,
}

let point = Point {
    x: 1,
    y: 2,
    z: 3,
    t: 4,
};

point.x_eq(&1).y_ne(&9).z_satisfies(|z| z % 3 == 0);
```
