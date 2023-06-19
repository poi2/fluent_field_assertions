use core::fmt::Debug;
use fluent_field_assertions::FluentFieldAssertions;

#[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
struct User {
    id: usize,
    name: String,
}

#[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
struct Container<T>
where
    T: Eq + Debug,
{
    value: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn alice() -> User {
        User {
            id: 1,
            name: "Alice".to_string(),
        }
    }

    fn hello_world() -> Container<String> {
        Container {
            value: "Hello, world!".to_string(),
        }
    }

    #[rstest]
    #[case::valid_name("Alice".to_string())]
    #[should_panic]
    #[case::invalid_name("Bob".to_string())]
    fn can_be_checked_equal_of_struct(#[case] expected: String) {
        alice().name_eq(expected);
    }

    #[rstest]
    #[should_panic]
    #[case::valid_name("Alice".to_string())]
    #[case::invalid_name("Bob".to_string())]
    fn can_be_checked_not_equal_of_struct(#[case] expected: String) {
        alice().name_ne(expected);
    }

    #[rstest]
    #[case::valid_value("Hello, world!".to_string())]
    #[should_panic]
    #[case::invalid_value("Hello, Rust!".to_string())]
    fn can_be_checked_equal_of_generics_struct(#[case] expected: String) {
        hello_world().value_eq(expected);
    }

    #[rstest]
    #[should_panic]
    #[case::valid_value("Hello, world!".to_string())]
    #[case::invalid_value("Hello, Rust!".to_string())]
    fn can_be_checked_not_equal_of_generics_struct(#[case] expected: String) {
        hello_world().value_ne(expected);
    }
}
