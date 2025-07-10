use crate::prelude::*;
use colored::{ColoredString, Colorize};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[allow(clippy::absolute_paths)]
#[derive(Debug)]
pub enum ExpectError {
    ExpectDirNotFound(PathBuf),
    CreateSubDir(std::io::Error, PathBuf),
    WriteActual(std::io::Error),
    CreateActual(std::io::Error, PathBuf),
    SerializeActual(Box<dyn Error>),
    FlushActual(std::io::Error),
    CopyActual(std::io::Error, PathBuf, PathBuf),
    OpenExpected(std::io::Error, PathBuf),
    ReadExpected(std::io::Error),
    DeserializeExpected(Box<dyn Error>),
}

impl Display for ExpectError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ExpectError::ExpectDirNotFound(path) => {
                format!("Expect directory not found:\n{}", format_path(path))
            }
            ExpectError::CreateSubDir(e, path) => {
                format!(
                    "Could not create test results directory: {}\n{}",
                    format_path(path),
                    format_error(e),
                )
            }
            ExpectError::WriteActual(e) => {
                format!("Could not write actual results file.\n{}", format_error(e),)
            }
            ExpectError::CreateActual(e, _) => {
                format!("Could not create actual results file.\n{}", format_error(e),)
            }
            ExpectError::SerializeActual(e) => {
                format!(
                    "Could not serialize actual results file.\n{}",
                    format_error(e.as_ref()),
                )
            }
            ExpectError::FlushActual(e) => {
                format!("Could not flush actual results file.\n{}", format_error(e),)
            }
            ExpectError::CopyActual(e, actual, expected) => {
                format!(
                    "Could not copy actual results file.\nFrom: {}\nTo: {}\n{}",
                    format_path(actual),
                    format_path(expected),
                    format_error(e),
                )
            }
            ExpectError::OpenExpected(e, _) => {
                format!("Could not open expected results file.\n{}", format_error(e),)
            }
            ExpectError::ReadExpected(e) => {
                format!("Could not read expected results file.\n{}", format_error(e),)
            }
            ExpectError::DeserializeExpected(e) => {
                format!(
                    "Could not deserialize expected results file.\n{}",
                    format_error(e.as_ref()),
                )
            }
        };
        write!(f, "{} to run test. {}", "Failed".bold(), message)
    }
}

fn format_path(path: &Path) -> ColoredString {
    path.display().to_string().dimmed()
}

fn format_error<E: Error + ?Sized>(error: &E) -> ColoredString {
    error.to_string().dimmed()
}
