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
    pub(crate) fn write_actual_json<T: Serialize>(
        &mut self,
        actual: &T,
    ) -> Result<(), ExpectError> {
        let mut writer = self.get_actual_writer(JSON_EXT)?;
        serde_json::to_writer_pretty(&mut writer, actual).map_err(ExpectError::SerializeActual)?;
        writer.flush().map_err(ExpectError::FlushActual)?;
        Ok(())
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
    fn write_actual_json() -> Result<(), ExpectError> {
        // Arrange
        let mut expect = Expect::new();
        let actual = SampleStruct::sample();
        let path = expect.get_actual_path(JSON_EXT);
        if path.exists() {
            remove_file(&path).expect("Should be able to remove file");
        }
        // Act
        expect.write_actual_json(&actual)?;
        // Assert
        let result = read_to_string(path).expect("Should be able to read file");
        assert!(!result.is_empty());
        Ok(())
    }
}
