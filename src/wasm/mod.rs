// Copyright (c) 2022 Sungbae Jeong
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voca {
    voca: Vec<Word>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    word: String,
    info: Vec<WordInfo>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordInfo {
    meaning: String,
    synos: Option<Vec<String>>,
    example: Option<String>,
}

#[wasm_bindgen]
impl Voca {
    pub fn new(toml_string: &str) -> Result<JsValue, JsValue> {
        let voca: Voca = match toml::from_str(toml_string) {
            Ok(voca) => voca,
            Err(err) => return Err(JsValue::from_str(Box::leak(Box::new(format!("{err}"))))),
        };

        Ok(serde_wasm_bindgen::to_value(&voca)?)
    }
}

#[wasm_bindgen]
pub fn rand(limit: usize) -> usize {
    rand::random::<usize>() % limit
}
