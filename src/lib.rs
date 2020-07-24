use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize, Serialize)]
pub struct FormatParams {
    pub values: Vec<Vec<String>>,
}

#[wasm_bindgen]
pub fn format(params: &JsValue) -> String {
    format_impl(params.into_serde::<FormatParams>().unwrap().values)
}

fn format_impl(s: Vec<Vec<String>>) -> String {
    let mut t = String::new();
    for (i, s_i) in s.iter().enumerate() {
        for (j, s_ij) in s_i.iter().enumerate() {
            for c in s_ij.chars() {
                match c {
                    '\n' => t.push_str("\\n"),
                    '\r' => t.push_str("\\r"),
                    '\t' => t.push_str("\\t"),
                    '\\' => t.push_str("\\\\"),
                    _ => t.push(c),
                }
            }
            if j < s_i.len() - 1 {
                t.push('\t');
            }
        }
        if i < s.len() - 1 {
            t.push('\n');
        }
    }
    t
}
