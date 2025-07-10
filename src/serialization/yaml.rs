use crate::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct YamlSerializer;

impl Serializer for YamlSerializer {
    const EXTENSION: &'static str = "yaml";

    fn serialize<W: Sized + Write, T: Serialize>(
        &self,
        mut writer: BufWriter<W>,
        value: &T,
    ) -> Result<(), ExpectError> {
        serde_yaml::to_writer(&mut writer, value)
            .map_err(|e| ExpectError::SerializeActual(Box::new(e)))?;
        writer.flush().map_err(ExpectError::FlushActual)?;
        Ok(())
    }

    fn deserialize<R: Read + Sized, T: DeserializeOwned>(
        &self,
        reader: BufReader<R>,
    ) -> Result<T, ExpectError> {
        serde_yaml::from_reader(reader).map_err(|e| ExpectError::DeserializeExpected(Box::new(e)))
    }
}
