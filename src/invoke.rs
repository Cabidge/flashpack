use std::collections::BTreeSet;

use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

macros::define_commands! {
    open_collection() -> bool;
    get_collection_name() -> Option<String>;
    list_packs() -> BTreeSet<String>;
    list_cards(pack_name: String) -> BTreeSet<String>;
    add_card(pack_name: String, card_name: String, contents: String);
    get_card(pack_name: String, card_name: String) -> Option<String>;
}

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

mod macros {
    macro_rules! define_command {
        ($name:ident($($arg_name:ident: $arg_ty:ty),*) $(-> $ret:ty)?) => {
            pub async fn $name($($arg_name: $arg_ty),*) $(-> $ret)? {
                camelpaste::paste! {
                    #[derive(serde::Serialize)]
                    #[allow(non_snake_case)]
                    struct Args {
                        $( [<$arg_name:camel>]: $arg_ty, )*
                    }

                    let args = Args {
                        $( [<$arg_name:camel>]: $arg_name, )*
                    };
                }

                crate::invoke::invoke(stringify!($name), &args).await.unwrap()
            }
        }
    }

    macro_rules! define_commands {
        ($($name:ident($($args:tt)*) $(-> $ret:ty)?);+ $(;)?) => {
            $(
                crate::invoke::macros::define_command! {
                    $name($($args)*) $(-> $ret)?
                }
            )+
        }
    }

    pub(crate) use define_command;
    pub(crate) use define_commands;
}
