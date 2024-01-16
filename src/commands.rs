use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = invoke)]
    pub async fn invoke_js_value(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn invoke<T: DeserializeOwned>(
    cmd: &str,
    args: &impl Serialize,
) -> Result<T, serde_wasm_bindgen::Error> {
    let args = serde_wasm_bindgen::to_value(args)?;
    let result = invoke_js_value(cmd, args).await;
    serde_wasm_bindgen::from_value(result)
}
