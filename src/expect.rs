use crate::diff::Diff;
use crate::prelude::*;
use std::fmt::Debug;
use std::panic::Location;

/// Compare tests results against expected values.
pub struct Expect {
    pub(crate) test: TestContext,
}

impl Expect {
    /// Create a new [`Expect`].
    #[track_caller]
    #[must_use]
    pub fn new() -> Self {
        let location = Location::caller();
        Self {
            test: TestContext::new(location),
        }
    }

    /// Compare a string with the expected value.
    pub fn string(&mut self, actual: &str, extension: &str) -> Result<bool, ExpectError> {
        self.verify_dirs()?;
        self.write_actual_text(actual, extension)?;
        let expected = self.read_expected_text(extension)?;
        Ok(Diff::string(actual, &expected))
    }

    /// Compare against the expected value.
    pub fn value<T: Debug + DeserializeOwned + PartialEq + Serialize>(
        &mut self,
        actual: &T,
    ) -> Result<bool, ExpectError> {
        self.verify_dirs()?;
        self.write_actual_json(&actual)?;
        let expected = self.read_expected_json::<T>()?;
        Ok(Diff::value(actual, &expected))
    }

    /// Compare against the expected values.
    pub fn values<T: Debug + DeserializeOwned + PartialEq + Serialize>(
        &mut self,
        actual: &[T],
    ) -> Result<bool, ExpectError> {
        self.verify_dirs()?;
        self.write_actual_json(&actual)?;
        let expected = self.read_expected_json::<Vec<T>>()?;
        Ok(Diff::values(actual, &expected))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() -> Result<(), ExpectError> {
        // Arrange
        let valid = "Hello, world!";
        let invalid = "Oh, no!";
        let mut expect = Expect::new();
        // Act
        // Assert
        assert!(expect.string(valid, TEXT_EXT)?, "Valid");
        assert!(!expect.string(invalid, TEXT_EXT)?, "Invalid");
        Ok(())
    }

    #[test]
    fn value() -> Result<(), ExpectError> {
        // Arrange
        let valid = SampleStruct::sample();
        let invalid = SampleStruct {
            string: "INVALID".to_owned(),
            ..SampleStruct::sample()
        };
        let mut expect = Expect::new();
        // Act
        // Assert
        assert!(expect.value(&valid)?, "Valid");
        assert!(!expect.value(&invalid)?, "Invalid");
        Ok(())
    }

    #[test]
    fn values() -> Result<(), ExpectError> {
        // Arrange
        let valid = SampleStruct::sample();
        let invalid = SampleStruct {
            string: "INVALID".to_owned(),
            ..SampleStruct::sample()
        };
        let mut expect = Expect::new();
        // Act
        // Assert
        assert!(expect.values(&vec![valid.clone(), valid.clone()])?, "Valid");
        assert!(!expect.values(&[valid.clone()])?, "Missing on actual");
        assert!(
            !expect.values(&vec![valid.clone(), valid.clone(), valid.clone()])?,
            "Missing on expected"
        );
        assert!(
            !expect.values(&vec![valid.clone(), invalid.clone()])?,
            "Invalid"
        );
        Ok(())
    }
}
