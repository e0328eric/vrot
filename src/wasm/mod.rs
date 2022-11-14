// Copyright (c) 2022 Sungbae Jeong
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voca {
    word: String,
    info: Vec<VocaInfo>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocaInfo {
    meaning: String,
    synos: Option<Vec<String>>,
    example: Option<String>,
}

#[wasm_bindgen]
impl Voca {
    pub fn new(yaml_string: &str) -> Result<JsValue, JsValue> {
        let vocas: Vec<Voca> = match serde_yaml::from_str(yaml_string) {
            Ok(vocas) => vocas,
            Err(err) => return Err(JsValue::from_str(Box::leak(Box::new(format!("{err}"))))),
        };

        Ok(serde_wasm_bindgen::to_value(&vocas)?)
    }
}

#[wasm_bindgen]
pub fn rand(limit: usize) -> usize {
    rand::random::<usize>() % limit
}