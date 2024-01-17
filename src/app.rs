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

    let pack_view = move || {
        pack_name.get().map(|pack_name| {
            let name = (move || pack_name.clone()).into_signal();
            let save_action = create_action(move |input: &(String, String)| {
                let (card_name, contents) = input.clone();
                async move {
                    #[derive(Serialize)]
                    struct Args {
                        packName: String,
                        cardName: String,
                        contents: String,
                    }

                    let args = Args {
                        packName: name.get(),
                        cardName: card_name,
                        contents,
                    };

                    invoke::<()>("add_card", &args).await.unwrap();
                }
            });

            let cards = create_resource(
                move || name.get(),
                |name| async move {
                    #[derive(Serialize)]
                    struct Args {
                        packName: String,
                    }

                    let args = Args {
                        packName: name.clone(),
                    };

                    invoke::<Vec<String>>("list_cards", &args).await.unwrap()
                },
            );

            view! {
                <button on:click=move |_| set_pack_name.set(None)>
                    "Back"
                </button>
                <Transition>
                    {move || {
                        let cards = move || cards.get().unwrap_or_default();
                        view! {
                            <Pack name cards on_save=move |input| save_action.dispatch(input)/>
                        }
                    }}
                </Transition>
            }
        })
    };

    view! {
        <h1>{name}</h1>
        <h2>"Packs"</h2>
        <ul>
            <li>
                <button on:click=move |_| open_pack(String::from("Foo"))>
                    "Foo"
                </button>
            </li>
        </ul>
        {pack_view}
    }
}

#[component]
fn Pack(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] cards: Signal<Vec<String>>,
    #[prop(into)] on_save: Callback<(String, String)>,
) -> impl IntoView {
    let selected_card = create_rw_signal(None::<String>);

    #[component]
    fn CardListItem(selected_card: RwSignal<Option<String>>, name: String) -> impl IntoView {
        let name = (move || name.clone()).into_signal();
        let is_selected = with!(|selected_card, name| selected_card.as_ref() == Some(name));

        let select = move || selected_card.set(Some(name.get()));

        view! {
            <li>
                <button class:selected=is_selected on:click=move |_| select()>
                    {name}
                </button>
            </li>
        }
    }

    let card_contents = create_resource(
        move || selected_card.get(),
        move |selected_card| async move {
            let card_name = selected_card?;
            Some(String::from("Foo bar baz"))
        },
    );

    let card_editor = move || {
        let data = selected_card
            .get()
            .and_then(|card| Some((card, card_contents.get().flatten()?)));

        if let Some((card_name, contents)) = data {
            let card_name = (move || card_name.clone()).into_signal();
            let (contents, set_contents) = create_signal(contents);

            let save = move |_| on_save.call((card_name.get(), contents.get()));

            view! {
                <h3>{card_name}</h3>
                <textarea
                    prop:value=move || contents.get()
                    on:input=move |ev| set_contents.set(event_target_value(&ev))
                >
                    {contents.get_untracked()}
                </textarea>
                <button on:click=save>
                    "Save"
                </button>
            }
            .into_view()
        } else {
            view! {
                <p>"No card selected..."</p>
            }
            .into_view()
        }
    };

    view! {
        <h1>{name}</h1>
        <ul>
            <For
                each=move || cards.get()
                key=|card| card.clone()
                let:name
            >
                <CardListItem selected_card name/>
            </For>
        </ul>
        <Transition>
            {card_editor}
        </Transition>
    }
}

#[component]
fn AddInput(#[prop(into)] on_add: Callback<String>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        on_add.call(input.get());
        set_input.update(|input| input.clear());
    };

    let on_input = move |ev: leptos::ev::Event| {
        set_input.set(event_target_value(&ev));
    };

    view! {
        <form on:submit=on_submit>
            <input
                type="text"
                prop:value=input
                on:input=on_input
            />
            <button type="submit">
                "Add"
            </button>
        </form>
    }
}

#[component]
fn CardSlides(card_slides: Vec<String>) -> impl IntoView {
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
