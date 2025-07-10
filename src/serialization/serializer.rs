use crate::prelude::*;

pub trait Serializer: Default {
    const EXTENSION: &'static str;

    /// Serialize a value.
    fn serialize<W: Sized + Write, T: Serialize>(
        &self,
        writer: BufWriter<W>,
        value: &T,
    ) -> Result<(), ExpectError>;

    /// Deserialize a value.
    fn deserialize<R: Read + Sized, T: DeserializeOwned>(
        &self,
        reader: BufReader<R>,
    ) -> Result<T, ExpectError>;
}

pub trait SerializerExtensions<S: Serializer> {
    fn get_extension(&self) -> &'static str;
    fn serialize_to_string<T: Serialize>(&self, value: T) -> Result<String, ExpectError>;
}

impl<S: Serializer> SerializerExtensions<S> for S {
    fn get_extension(&self) -> &'static str {
        Self::EXTENSION
    }

    fn serialize_to_string<T: Serialize>(&self, value: T) -> Result<String, ExpectError> {
        let mut buffer = Vec::new();
        {
            let writer = BufWriter::new(&mut buffer);
            self.serialize(writer, &value)?;
        }
        let serialized = String::from_utf8(buffer).expect("Should be valid UTF-8");
        Ok(serialized)
    }
}
