use leptos::*;
use leptos_router::*;

pub fn use_pack_name() -> Signal<String> {
    #[derive(Params, PartialEq, Clone)]
    struct UrlParam {
        pack_name: Option<String>,
    }

    let param = use_params::<UrlParam>();

    Signal::derive(move || {
        param.with(|params| {
            let Some(name) = params
                .as_ref()
                .ok()
                .and_then(|params| params.pack_name.as_ref())
            else {
                return String::from(":pack_name");
            };

            percent_encoding::percent_decode_str(name)
                .decode_utf8_lossy()
                .into_owned()
        })
    })
}

pub fn use_card_name() -> Signal<String> {
    #[derive(Params, PartialEq, Clone)]
    struct UrlParam {
        card_name: Option<String>,
    }

    let param = use_params::<UrlParam>();

    Signal::derive(move || {
        param.with(|params| {
            let Some(name) = params
                .as_ref()
                .ok()
                .and_then(|params| params.card_name.as_ref())
            else {
                return String::from(":card_name");
            };

            percent_encoding::percent_decode_str(name)
                .decode_utf8_lossy()
                .into_owned()
        })
    })
}
