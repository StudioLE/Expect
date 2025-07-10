use crate::prelude::*;

impl Expect {
    /// Read the expected results file as text to a string.
    pub(crate) fn read_expected_text(&mut self, extension: &str) -> Result<String, ExpectError> {
        let mut reader = self.get_expected_reader(extension)?;
        let mut text = String::new();
        reader
            .read_to_string(&mut text)
            .map_err(ExpectError::ReadExpected)?;
        Ok(text)
    }

    /// Read and deserialize the expected results file.
    pub(crate) fn read_expected_serialized<T: DeserializeOwned>(
        &mut self,
    ) -> Result<T, ExpectError> {
        let serializer = DefaultSerializer::get();
        let reader = self.get_expected_reader(serializer.get_extension())?;
        serializer.deserialize(reader)
    }

    /// Get a [`BufReader`] for the expected results file.
    fn get_expected_reader(&mut self, extension: &str) -> Result<BufReader<File>, ExpectError> {
        let path = self.get_expected_path(extension);
        if !path.is_file() {
            println!("Creating expected file: {}", path.display());
            let actual = self.get_actual_path(extension);
            copy(&actual, &path).map_err(|e| ExpectError::CopyActual(e, actual, path.clone()))?;
        }
        let file = File::open(&path).map_err(|e| ExpectError::OpenExpected(e, path))?;
        Ok(BufReader::new(file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_expected_text() -> Result<(), ExpectError> {
        // Arrange
        let mut expect = Expect::new();
        let expected = "Hello, world!";
        // Act
        let result = expect.read_expected_text(TEXT_EXT)?;
        // Assert
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn read_expected_serialized() -> Result<(), ExpectError> {
        // Arrange
        let mut expect = Expect::new();
        let expected = SampleStruct::sample();
        // Act
        let result: SampleStruct = expect.read_expected_serialized()?;
        // Assert
        assert_eq!(result, expected);
        Ok(())
    }
}
