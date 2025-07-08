use crate::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SampleStruct {
    pub string: String,
    pub integer: i32,
    pub float: f32,
    pub bool: bool,
    pub r#enum: SampleEnum,
    pub vec: Vec<f32>,
    pub hash_map: HashMap<i32, f32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum SampleEnum {
    A,
    B,
    C,
}

impl SampleStruct {
    pub(crate) fn sample() -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert(1, 1.0);
        hash_map.insert(2, 1.0 / 3.0);
        hash_map.insert(3, PI);
        Self {
            string: "Hello, world!".to_owned(),
            integer: 1,
            float: 7.2,
            bool: true,
            r#enum: SampleEnum::B,
            vec: vec![11.1, 2.0, 3.0],
            hash_map,
        }
    }
}
