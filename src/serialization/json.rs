use crate::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct JsonSerializer;

impl Serializer for JsonSerializer {
    const EXTENSION: &'static str = "json";

    fn serialize<W: Sized + Write, T: Serialize>(
        &self,
        mut writer: BufWriter<W>,
        value: &T,
    ) -> Result<(), ExpectError> {
        serde_json::to_writer_pretty(&mut writer, value)
            .map_err(|e| ExpectError::SerializeActual(Box::new(e)))?;
        writer.flush().map_err(ExpectError::FlushActual)?;
        Ok(())
    }

    fn deserialize<R: Read + Sized, T: DeserializeOwned>(
        &self,
        reader: BufReader<R>,
    ) -> Result<T, ExpectError> {
        serde_json::from_reader(reader).map_err(|e| ExpectError::DeserializeExpected(Box::new(e)))
    }
}
