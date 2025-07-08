use std::panic::Location;
use std::path::PathBuf;
use std::thread;

/// Context of the current test.
#[derive(Clone, Debug)]
pub(crate) struct TestContext {
    /// Module hierarchy of the current test.
    ///
    /// Excludes the test name.
    #[allow(dead_code)]
    pub(crate) module: Vec<String>,
    /// Name of the test excluding the path.
    pub(crate) name: String,
    /// Path of the file containing the test relative to the root directory.
    pub(crate) file: PathBuf,
    /// Line number of the test.
    #[allow(dead_code)]
    pub(crate) line: u32,
}

impl TestContext {
    /// Create a new [`TestContext`].
    pub(crate) fn new(location: &Location) -> Self {
        let name = thread::current()
            .name()
            .expect("Should be able to get test name")
            .to_owned();
        let components: Vec<String> = name.split("::").map(ToOwned::to_owned).collect();
        let module = components
            .iter()
            .take(components.len() - 1)
            .map(ToOwned::to_owned)
            .collect();
        let name = components
            .last()
            .expect("Should be at least one component in test name")
            .to_owned();
        Self {
            module,
            name,
            file: PathBuf::from(location.file()),
            line: location.line(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // Arrange
        let line = line!() + 2;
        // Act
        let location = Location::caller();
        let context = TestContext::new(location);
        // Assert
        println!("{context:?}");
        assert_eq!(
            context.module,
            vec!["context".to_owned(), "tests".to_owned()],
            "Module path"
        );
        assert_eq!(context.name, "new".to_owned(), "Test name");
        assert_eq!(context.file, PathBuf::from("src/context.rs"), "File path");
        assert!(context.file.is_file(), "File path");
        assert_eq!(context.line, line, "Line number");
    }
}
