use leptos::*;
use leptos_router::*;

pub fn use_pack_name() -> Signal<String> {
    #[derive(Params, PartialEq, Clone)]
    struct UrlParam {
        pack_name: Option<String>,
    }

    derive_param::<UrlParam>(|params| params.pack_name.as_ref())
}

pub fn use_card_name() -> Signal<String> {
    #[derive(Params, PartialEq, Clone)]
    struct UrlParam {
        card_name: Option<String>,
    }

    derive_param::<UrlParam>(|params| params.card_name.as_ref())
}

// this is a little dumb looking, but it just removes a lot of repetitive code
fn derive_param<T: Params + PartialEq + 'static>(map: fn(&T) -> Option<&String>) -> Signal<String> {
    let param = use_params::<T>();

    Signal::derive(move || {
        param.with(move |param| {
            let Some(name) = param.as_ref().ok().and_then(map) else {
                return String::new();
            };

            percent_encoding::percent_decode_str(name)
                .decode_utf8_lossy()
                .into_owned()
        })
    })
}
