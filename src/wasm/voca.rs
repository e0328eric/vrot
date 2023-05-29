use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

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
#[derive(Serialize, Deserialize)]
pub struct HashSet {
    inner: std::collections::HashSet<usize>,
}

#[wasm_bindgen]
impl Voca {
    pub fn new(toml_string: &str) -> JsValue {
        let voca: Voca = match toml::from_str(toml_string) {
            Ok(voca) => voca,
            Err(err) => {
                alert(&format!(
                    "Failed to parse a toml files. Check that members name are correct.\n{err}"
                ));
                return JsValue::NULL;
            }
        };

        match serde_wasm_bindgen::to_value(&voca) {
            Ok(val) => val,
            Err(err) => {
                alert(&format!("{err}"));
                JsValue::NULL
            }
        }
    }
}

#[wasm_bindgen]
impl HashSet {
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashSet::new(),
        }
    }

    pub fn insert(&mut self, item: usize) {
        self.inner.insert(item);
    }

    pub fn contains(&self, item: usize) -> bool {
        self.inner.contains(&item)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

#[wasm_bindgen]
pub fn rand(limit: usize) -> usize {
    rand::random::<usize>() % limit
}
