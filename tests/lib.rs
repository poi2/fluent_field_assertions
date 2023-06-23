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

    #[test]
    fn satisfies_works_properly() {
        hello_world().value_satisfies(|value| value.contains("world"));
    }

    #[test]
    fn uses_in_readme_work_properly() {
        use fluent_field_assertions::FluentFieldAssertions;

        #[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
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
        user.id_eq(1)
            .name_eq("Alice".to_string())
            .age_satisfies(|age| age < &18);

        // Same as above.
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice".to_string());
        assert!(user.age < 18);
    }

    #[test]
    fn struct_examples_in_readme_work_properly() {
        use fluent_field_assertions::FluentFieldAssertions;

        #[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
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

        // You can use `{field}_eq` to assert_eq!.
        user.name_eq("Alice".to_string());
        assert_eq!(user.name, "Alice".to_string()); // Same as above.

        // You can use `{field}_ne` to assert_ne!.
        user.name_ne("Bob".to_string());
        assert_ne!(user.name, "Bob".to_string()); // Same as above.

        // You can use `{field}_satisfies` to assert!.
        user.name_satisfies(|name| name.starts_with('A'));
        assert!(user.name.starts_with('A')); // Same as above.

        // You can chain assertions as method calls.
        user.id_eq(1)
            .name_eq("Alice".to_string())
            .age_satisfies(|age| age < &18);

        // Same as above.
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice".to_string());
        assert!(user.age < 18);
    }

    #[test]
    fn generic_examples_in_readme_work_properly() {
        use core::fmt::Debug;
        use fluent_field_assertions::FluentFieldAssertions;

        #[derive(FluentFieldAssertions, Debug, Eq, PartialEq)]
        struct Point<T>
        // Generics type `T` must implement trait `Eq` and `Display`.
        where
            T: Eq + Debug,
        {
            x: T,
            y: T,
            z: T,
        }

        let point = Point { x: 1, y: 2, z: 3 };

        point.x_eq(1).y_ne(9).z_satisfies(|z| z % 3 == 0);
    }
}
