mod params;

use leptos::*;
use leptos_router::*;

use crate::{context::CollectionName, invoke};

#[component]
pub fn App() -> impl IntoView {
    let open_collection_action = create_action(|_: &()| invoke::open_collection());

    let collection_name = create_memo(move |prev| {
        open_collection_action
            .value()
            .get()
            .flatten()
            .or_else(|| prev.cloned().flatten())
    });

    provide_context(CollectionName(collection_name.into()));

    let title = move || {
        collection_name
            .get()
            .unwrap_or_else(|| String::from("No Collection Selected..."))
    };

    view! {
        <Router>
            <main>
                <button on:click=move |_| open_collection_action.dispatch(())>
                    "Open Collection"
                </button>
                <h1>{title}</h1>
                <Routes>
                    <Route path="" view=PackList/>
                    <Route path="/pack/:pack_name" view=Pack>
                        <Route path="/" view=|| view! { <h2>"No Card Selected..."</h2> }/>
                        <Route path="/card/:card_name" view=CardEditor/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn PackList() -> impl IntoView {
    let CollectionName(collection_name) = use_context::<CollectionName>().unwrap_or_default();

    let packs = create_resource(
        move || collection_name.get(), // TODO: make save actoin a dependency
        |_| invoke::list_packs(),
    );

    let save_action = create_action(move |input: &(String, String, String)| {
        let (pack_name, card_name, contents) = input.clone();
        invoke::add_card(pack_name, card_name, contents)
    });

    let pack_list_view = move || {
        packs.get().map(move |packs| {
            packs
                .into_iter()
                .map(move |pack_name| {
                    let href = format!("pack/{pack_name}");

                    view! {
                        <li>
                            <A href>{pack_name}</A>
                        </li>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <Show when=move || collection_name.get().is_some()>
            <h2>"Packs"</h2>
            <ul>
                <Transition>
                    {pack_list_view}
                </Transition>
            </ul>
            <AddInput on_add=move |_name| todo!()/>
        </Show>
        <Outlet/>
    }
}

#[component]
fn Pack() -> impl IntoView {
    let params = params::use_pack_params();

    let name = move || params.with(|params| params.pack().map(str::to_string));
    let selected_card = move || params.with(|params| params.card().map(str::to_string));

    let cards = create_resource(
        move || {
            // TODO: reload on save
            name()
        },
        move |pack_name| async move {
            let Some(pack_name) = pack_name else {
                return vec![];
            };

            invoke::list_cards(pack_name).await
        },
    );

    let card_list = move || {
        cards
            .get()
            .map(|cards| view! { <CardList cards selected_card/> })
    };

    view! {
        <a href="/">"Back"</a>
        <h2>{name}</h2>
        <Transition>
            {card_list}
        </Transition>
        <Outlet/>
    }
}

#[component]
fn CardEditor() -> impl IntoView {
    let params = params::use_pack_params();

    let card_name = move || params.with(|params| params.card().map(str::to_string));

    #[component]
    fn Editor(
        #[prop(into)] initial_contents: String,
        #[prop(into)] on_save: Callback<String>,
    ) -> impl IntoView {
        let (contents, set_contents) = create_signal(initial_contents);

        view! {
            <textarea
                prop:value=move || contents.get()
                on:input=move |ev| set_contents.set(event_target_value(&ev))
            >
                {contents.get_untracked()}
            </textarea>
            <button on:click=move |_| on_save.call(contents.get())>
                "Save"
            </button>
        }
    }

    let contents = create_resource(
        move || params.with(|params| params.both().map(|(a, b)| (a.to_string(), b.to_string()))),
        |params| async move {
            let Some((pack_name, card_name)) = params else {
                return String::new();
            };

            invoke::get_card(pack_name, card_name)
                .await
                .unwrap_or_default()
        },
    );

    let editor = move || {
        contents.get().map(|initial_contents| {
            view! {
                <Editor initial_contents on_save=|_| ()/>
            }
        })
    };

    view! {
        <h3>{card_name}</h3>
        <Transition>
            {editor}
        </Transition>
    }
}

#[component]
fn CardList(
    #[prop(into)] cards: MaybeSignal<Vec<String>>,
    #[prop(into)] selected_card: Signal<Option<String>>,
) -> impl IntoView {
    #[component]
    fn CardListItem(
        #[prop(into)] is_selected: Signal<bool>,
        #[prop(into)] name: String,
    ) -> impl IntoView {
        let href = format!("card/{name}");
        view! {
            <li class:selected=is_selected>
                <A href>
                    {name}
                </A>
            </li>
        }
    }

    view! {
        <ul>
            <For
                each=move || cards.get()
                key=|card| card.clone()
                children=move |name| {
                    let card_name = name.clone();
                    let is_selected = move || {
                        selected_card.with(|selected| selected.as_ref() == Some(&card_name))
                    };

                    view! {
                        <CardListItem is_selected name/>
                    }
                }
            />
        </ul>
        <AddInput on_add=move |_new_card| todo!()/>
    }
}

#[component]
fn AddInput(#[prop(into)] on_add: Callback<String>) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if input.with(|input| input.chars().all(char::is_whitespace)) {
            return;
        }

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
