use crate::prelude::*;

#[cfg(feature = "yaml")]
pub(crate) type DefaultSerializer = YamlSerializer;
#[cfg(all(not(feature = "yaml"), feature = "json"))]
pub(crate) type DefaultSerializer = JsonSerializer;
