use crate::prelude::*;

pub(crate) struct DefaultSerializer;

impl DefaultSerializer {
    #[cfg(feature = "yaml")]
    pub(crate) fn get() -> YamlSerializer {
        YamlSerializer
    }

    #[cfg(all(not(feature = "yaml"), feature = "json"))]
    pub(crate) fn get() -> JsonSerializer {
        JsonSerializer
    }

    #[cfg(all(not(feature = "yaml"), not(feature = "json")))]
    pub(crate) fn get() -> impl Serializer {
        compile_error!("Either the 'yaml' or 'json' feature must be enabled");
    }
}
