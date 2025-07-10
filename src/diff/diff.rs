use crate::prelude::*;

pub(crate) struct Diff;

impl Diff {
    /// Compare strings and print the differences.
    pub(crate) fn string(actual: &str, expected: &str) -> bool {
        let is_success = actual == expected;
        if !is_success {
            print_actual(actual);
            print_expected(expected);
        }
        is_success
    }

    /// Compare with [`PartialEq`] and print the differences.
    pub(crate) fn value<T: Debug + DeserializeOwned + PartialEq + Serialize>(
        actual: &T,
        expected: &T,
    ) -> bool {
        let is_success = actual == expected;
        if !is_success {
            print_actual(&display_value(&actual));
            print_expected(&display_value(&expected));
        }
        is_success
    }

    /// Compare each value with [`PartialEq`] and print the differences.
    pub(crate) fn values<T: Debug + DeserializeOwned + PartialEq + Serialize>(
        actual: &[T],
        expected: &[T],
    ) -> bool {
        let max = if actual.len() > expected.len() {
            actual.len()
        } else {
            expected.len()
        };
        let mut is_success = true;
        for i in 0..max {
            let Some(actual_item) = actual.get(i) else {
                println!("Index {i}");
                let expected_item = expected.get(i).expect("Verified item should exist");
                print_actual("[No item at index]");
                print_expected(&display_value(&expected_item));
                is_success = false;
                continue;
            };
            let Some(expected_item) = expected.get(i) else {
                println!("Index {i}");
                print_actual(&display_value(&actual_item));
                print_expected("[No item at index]");
                is_success = false;
                continue;
            };
            if actual_item != expected_item {
                println!("Index {i}");
                print_actual(&display_value(&actual_item));
                print_expected(&display_value(&expected_item));
                is_success = false;
            }
        }
        is_success
    }
}

fn display_value<T: Debug + Serialize>(value: &T) -> String {
    let serializer = DefaultSerializer::default();
    serializer
        .serialize_to_string(value)
        .unwrap_or_else(|_| format!("{value:?}"))
}

fn print_actual(actual: &str) {
    println!("{}", actual.red());
}

fn print_expected(expected: &str) {
    println!("{}", expected.green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        // Arrange
        let valid = "Hello, world!";
        let invalid = "Oh, no!";
        // Act
        // Assert
        assert!(Diff::string(valid, valid), "Valid");
        assert!(!Diff::string(invalid, valid), "Invalid");
    }

    #[test]
    fn value() {
        // Arrange
        let valid = SampleStruct::sample();
        let invalid = SampleStruct {
            string: "INVALID".to_owned(),
            ..SampleStruct::sample()
        };
        // Act
        // Assert
        assert!(Diff::value(&valid, &valid), "Valid");
        assert!(!Diff::value(&invalid, &valid), "Invalid");
    }

    #[test]
    fn values() {
        // Arrange
        let valid = SampleStruct::sample();
        let invalid = SampleStruct {
            string: "INVALID".to_owned(),
            ..SampleStruct::sample()
        };
        // Act
        // Assert
        assert!(
            Diff::values(
                &vec![valid.clone(), valid.clone()],
                &vec![valid.clone(), valid.clone()]
            ),
            "Valid"
        );
        assert!(
            !Diff::values(&[valid.clone()], &vec![valid.clone(), valid.clone()]),
            "Missing on actual"
        );
        assert!(
            !Diff::values(&vec![valid.clone(), valid.clone()], &[valid.clone()]),
            "Missing on expected"
        );
        assert!(
            !Diff::values(
                &vec![valid.clone(), invalid.clone()],
                &vec![valid.clone(), valid.clone()]
            ),
            "Invalid"
        );
    }
}
