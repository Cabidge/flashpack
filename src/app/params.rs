use leptos::*;
use leptos_router::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PackParams {
    None,
    Pack(String),
    PackAndCard(String, String),
}

pub fn use_pack_params() -> Memo<PackParams> {
    #[derive(Params, PartialEq, Default, Clone)]
    struct UrlParams {
        pack_name: Option<String>,
        card_name: Option<String>,
    }

    let params = use_params::<UrlParams>();

    create_memo(move |_| {
        fn decode(name: &str) -> Option<String> {
            let decoded = urlencoding::decode(name).ok()?;
            Some(decoded.into_owned())
        }

        let Ok(params) = params.get() else {
            return PackParams::None;
        };

        let Some(pack_name) = params.pack_name.as_deref().and_then(decode) else {
            return PackParams::None;
        };

        match params.card_name.as_deref().and_then(decode) {
            Some(card_name) => PackParams::PackAndCard(pack_name, card_name),
            None => PackParams::Pack(pack_name),
        }
    })
}

impl PackParams {
    pub fn pack(&self) -> Option<&str> {
        match self {
            Self::Pack(name) | Self::PackAndCard(name, _) => Some(name),
            _ => None,
        }
    }

    pub fn card(&self) -> Option<&str> {
        self.both().map(|(_, card)| card)
    }

    pub fn both(&self) -> Option<(&str, &str)> {
        match self {
            Self::PackAndCard(pack, card) => Some((pack, card)),
            _ => None,
        }
    }
}
