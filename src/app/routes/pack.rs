use std::collections::BTreeSet;

use leptos::*;
use leptos_router::*;

use crate::{app::AddInput, context, invoke, params};

// "/pack/:pack_name"
#[component]
pub fn PackPage() -> impl IntoView {
    let name = params::use_pack_name();

    let save_action = context::SaveAction::use_context().unwrap();

    let cards = create_resource(
        move || (name.get(), save_action.version().get()),
        |(name, _)| invoke::list_cards(name),
    );

    let card_list = move || cards.get().map(|cards| view! { <CardList cards/> });

    view! {
        <header>
            <a class="back-button" href="/">"<"</a>
            <h1>{name}</h1>
            <A href="study">"Begin study"</A>
        </header>
        <main class="pack-view">
            <div class="sidebar">
                <Transition>
                    {card_list}
                </Transition>
            </div>
            <div class="editor-window">
                <Outlet/>
            </div>
        </main>
    }
}

#[component]
fn CardList(#[prop(into)] cards: MaybeSignal<BTreeSet<String>>) -> impl IntoView {
    let route = use_route();
    let path = route.path();
    let navigate = use_navigate();

    let add_card = move |card_name| {
        navigate(&format!("{path}/card/{card_name}"), Default::default());
    };

    let card_list_item = move |name: String| {
        let href = format!(
            "card/{}",
            percent_encoding::utf8_percent_encode(&name, percent_encoding::NON_ALPHANUMERIC)
        );

        view! {
            <li class="card-list-item">
                <A exact=true active_class="selected" href>
                    {name}
                </A>
            </li>
        }
    };

    view! {
        <ul class="card-list">
            <For
                each=move || cards.get()
                key=|card| card.clone()
                children=card_list_item
            />
        </ul>
        <AddInput on_add=add_card/>
    }
}
