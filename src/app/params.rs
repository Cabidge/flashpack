use leptos::*;
use leptos_router::*;

pub fn use_pack_name() -> Signal<String> {
    #[derive(Params, PartialEq, Default, Clone)]
    struct PackNameParam {
        pack_name: Option<String>,
    }

    let param = use_params::<PackNameParam>();

    Signal::derive(move || {
        param.with(|params| {
            let name = params
                .as_ref()
                .unwrap()
                .pack_name
                .as_ref()
                .expect(":pack_name");

            urlencoding::decode(name).unwrap().into_owned()
        })
    })
}

pub fn use_card_name() -> Signal<String> {
    #[derive(Params, PartialEq, Default, Clone)]
    struct CardNameParam {
        card_name: Option<String>,
    }

    let param = use_params::<CardNameParam>();

    Signal::derive(move || {
        param.with(|params| {
            let name = params
                .as_ref()
                .unwrap()
                .card_name
                .as_ref()
                .expect(":card_name");

            urlencoding::decode(name).unwrap().into_owned()
        })
    })
}
