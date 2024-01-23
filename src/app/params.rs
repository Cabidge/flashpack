use leptos::*;
use leptos_router::*;

pub fn use_pack_name() -> Signal<String> {
    use_param_decoded("pack_name")
}

pub fn use_card_name() -> Signal<String> {
    use_param_decoded("card_name")
}

pub fn use_param_decoded(name: &'static str) -> Signal<String> {
    let params = use_params_map();

    Signal::derive(move || {
        params.with(|params| {
            let Some(name) = params.get(name) else {
                return String::new();
            };

            percent_encoding::percent_decode_str(name)
                .decode_utf8_lossy()
                .into_owned()
        })
    })
}
