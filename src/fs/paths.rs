use crate::prelude::*;

pub(crate) const EXPECT_DIR: &str = ".expect";
pub(crate) const ACTUAL_EXT: &str = "actual";
pub(crate) const EXPECT_EXT: &str = "expect";
pub(crate) const JSON_EXT: &str = "json";
#[cfg(test)]
pub(crate) const TEXT_EXT: &str = "txt";

impl Expect {
    /// Get the expect directory path for the module.
    ///
    /// Example: `src/path/to/module/.expect`
    fn get_module_dir(&self) -> PathBuf {
        let dir = self
            .test
            .file
            .parent()
            .expect("Test file should have a parent directory");
        dir.join(EXPECT_DIR)
    }

    /// Get the expect directory path for the current test.
    ///
    /// Example: `src/path/to/module/.expect`
    fn get_test_dir(&self) -> PathBuf {
        let file_stem = self
            .test
            .file
            .file_stem()
            .expect("Test file should have a file stem");
        self.get_module_dir().join(file_stem)
    }

    /// Get the results directory path for the current test.
    fn get_test_file(&self, extension: &str) -> PathBuf {
        self.get_test_dir()
            .join(format!("{}.{extension}", self.test.name))
    }

    /// Get the path of the actual test results.
    pub(super) fn get_actual_path(&self, extension: &str) -> PathBuf {
        self.get_test_file(&format!("{ACTUAL_EXT}.{extension}"))
    }

    /// Get the path of the expected test results.
    pub(super) fn get_expected_path(&self, extension: &str) -> PathBuf {
        self.get_test_file(&format!("{EXPECT_EXT}.{extension}"))
    }

    pub(crate) fn verify_dirs(&mut self) -> Result<(), ExpectError> {
        let module_dir = self.get_module_dir();
        if !module_dir.is_dir() {
            return Err(ExpectError::ExpectDirNotFound(module_dir));
        }
        let test_dir = self.get_test_dir();
        if !test_dir.is_dir() {
            create_dir_all(&test_dir).map_err(|e| ExpectError::CreateSubDir(e, test_dir))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_expect_dir() {
        // Arrange
        let expect = Expect::new();
        // Act
        let path = expect.get_module_dir();
        // Assert
        assert_eq!(path, PathBuf::from(format!("src/fs/{EXPECT_DIR}")));
    }

    #[test]
    fn get_results_dir_for_test() {
        // Arrange
        let expect = Expect::new();
        // Act
        let path = expect.get_test_dir();
        // Assert
        assert_eq!(path, PathBuf::from(format!("src/fs/{EXPECT_DIR}/paths")));
    }

    #[test]
    fn get_actual_path() {
        // Arrange
        let expect = Expect::new();
        // Act
        let path = expect.get_actual_path(JSON_EXT);
        // Assert
        assert_eq!(
            path,
            PathBuf::from(format!(
                "src/fs/{EXPECT_DIR}/paths/get_actual_path.{ACTUAL_EXT}.{JSON_EXT}"
            ))
        );
    }

    #[test]
    fn get_expected_path() {
        // Arrange
        let expect = Expect::new();
        // Act
        let path = expect.get_expected_path(JSON_EXT);
        // Assert
        assert_eq!(
            path,
            PathBuf::from(format!(
                "src/fs/{EXPECT_DIR}/paths/get_expected_path.{EXPECT_EXT}.{JSON_EXT}"
            ))
        );
    }
}
