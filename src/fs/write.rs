use crate::prelude::*;

impl Expect {
    /// Write the actual results to a file.
    pub(crate) fn write_actual_text(
        &mut self,
        actual: &str,
        extension: &str,
    ) -> Result<(), ExpectError> {
        let mut writer = self.get_actual_writer(extension)?;
        writer
            .write_all(actual.as_bytes())
            .map_err(ExpectError::WriteActual)?;
        Ok(())
    }

    /// Serialize the actual results and write to a file.
    pub(crate) fn write_actual_serialized<T: Serialize>(
        &mut self,
        actual: &T,
    ) -> Result<(), ExpectError> {
        let serializer = DefaultSerializer::default();
        let writer = self.get_actual_writer(serializer.get_extension())?;
        serializer.serialize(writer, actual)
    }

    /// Get a [`BufWriter`] for the actual results file.
    fn get_actual_writer(&mut self, extension: &str) -> Result<BufWriter<File>, ExpectError> {
        let path = self.get_actual_path(extension);
        let file = File::create(&path).map_err(|e| ExpectError::CreateActual(e, path))?;
        Ok(BufWriter::new(file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_actual_text() -> Result<(), ExpectError> {
        // Arrange
        let mut expect = Expect::new();
        let actual = "Hello, world!";
        let path = expect.get_actual_path(TEXT_EXT);
        if path.exists() {
            remove_file(&path).expect("Should be able to remove file");
        }
        // Act
        expect.write_actual_text(actual, TEXT_EXT)?;
        // Assert
        let result = read_to_string(path).expect("Should be able to read file");
        assert_eq!(result, actual);
        Ok(())
    }

    #[test]
    fn write_actual_serialized() -> Result<(), ExpectError> {
        // Arrange
        let mut expect = Expect::new();
        let actual = SampleStruct::sample();
        let extension = DefaultSerializer::default().get_extension();
        let path = expect.get_actual_path(extension);
        if path.exists() {
            remove_file(&path).expect("Should be able to remove file");
        }
        // Act
        expect.write_actual_serialized(&actual)?;
        // Assert
        let result = read_to_string(path).expect("Should be able to read file");
        assert!(!result.is_empty());
        Ok(())
    }
}
