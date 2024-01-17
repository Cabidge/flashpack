use leptos::*;
use serde::Serialize;

use crate::commands::invoke;

#[component]
pub fn App() -> impl IntoView {
    let open_collection_action = create_action(|_: &()| async {
        invoke::<Option<String>>("open_collection", &())
            .await
            .unwrap()
    });

    let collection_name = create_memo(move |prev| {
        open_collection_action
            .value()
            .get()
            .flatten()
            .or_else(|| prev.cloned().flatten())
    });

    let collection_view = move || match collection_name.get() {
        Some(name) => view! { <Collection name/> }.into_view(),
        None => view! { <h1>"No Collection Selected..."</h1> }.into_view(),
    };

    view! {
        <main>
            <button on:click=move |_| open_collection_action.dispatch(())>
                "Open Collection"
            </button>
            {collection_view}
        </main>
    }
}

#[component]
fn Collection(name: String) -> impl IntoView {
    let (pack_name, set_pack_name) = create_signal(None::<String>);

    let open_pack = move |name| set_pack_name.set(Some(name));

    let contents = move || match pack_name.get() {
        Some(name) => view! {
            <button on:click=move |_| set_pack_name.set(None)>
                "Back"
            </button>
            <Pack name/>
        }
        .into_view(),
        None => view! {
            <h2>"Packs"</h2>
            <ul>
                <li>
                    <button on:click=move |_| open_pack(String::from("Foo"))>
                        "Foo"
                    </button>
                </li>
            </ul>
        }
        .into_view(),
    };

    view! {
        <h1>{name}</h1>
        {contents}
    }
}

#[component]
fn Pack(name: String) -> impl IntoView {
    async fn list_cards(name: String) -> Vec<String> {
        #[derive(Serialize)]
        struct Args {
            packName: String,
        }

        let args = Args { packName: name };

        invoke("list_cards", &args).await.unwrap()
    }

    let cards = create_resource(move || (), {
        let name = name.clone();
        move |_| list_cards(name.clone())
    });

    let card_slides = vec![
        String::from("Hello"),
        String::from("Foo"),
        String::from("Bar"),
    ];

    view! {
        <Card card_slides/>
    }
}

#[component]
fn Card(card_slides: Vec<String>) -> impl IntoView {
    let (visible_count, set_visible_count) = create_signal(1);
    let slide_count = card_slides.len();

    let advance_slides = move || set_visible_count.update(|count| *count += 1);

    let visible_cards = move || {
        card_slides
            .iter()
            .take(visible_count.get())
            .map(|slide| {
                view! {
                    <li>
                        <div inner_html=slide></div>
                    </li>
                }
            })
            .collect_view()
    };

    view! {
        <ul>
            {visible_cards}
            <Show when=move || visible_count.get() < slide_count>
                <li>
                    <button on:click=move |_| advance_slides()>
                        "Next"
                    </button>
                </li>
            </Show>
        </ul>
    }
}
