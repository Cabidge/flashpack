mod params;

use std::collections::BTreeSet;

use leptos::*;
use leptos_router::*;

use crate::{context, invoke};

#[component]
pub fn App() -> impl IntoView {
    let save_action = create_action(|input: &(String, String, String)| {
        let (pack_name, card_name, contents) = input.clone();
        invoke::add_card(pack_name, card_name, contents)
    });

    context::SaveAction::provide(save_action);

    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=PackList/>
                    <Route path="/pack/:pack_name" view=Pack>
                        <Route path="/" view=|| view! { <h2>"No Card Selected..."</h2> }/>
                        <Route path="/card/:card_name" view=CardEditor/>
                    </Route>
                    // We don't want to show the CardList when in study mode
                    <Route path="/pack/:pack_name/study" view=Study/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn PackList() -> impl IntoView {
    let save_action = context::SaveAction::use_context().unwrap();

    let open_collection_action = create_action(|_: &()| invoke::open_collection());

    let collection_name = create_resource(
        move || open_collection_action.version().get(),
        |_| invoke::get_collection_name(),
    );

    let title = move || {
        collection_name
            .get()
            .flatten()
            .unwrap_or_else(|| String::from("No Collection Selected..."))
    };

    let packs = create_resource(
        move || (save_action.version().get(), collection_name.get()),
        |_| invoke::list_packs(),
    );

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

    let navigate = use_navigate();

    // need to create a Callback here because otherwise the compiler
    // complains for some reason, and i don't care enough to figure out why
    let add_pack = Callback::new(move |pack_name| {
        navigate(&format!("/pack/{pack_name}"), Default::default());
    });

    view! {
        <h1>{title}</h1>
        <button on:click=move |_| open_collection_action.dispatch(())>
            "Open Collection"
        </button>
        <Show when=move || collection_name.with(|name| matches!(name, Some(Some(_))))>
            <h2>"Packs"</h2>
            <ul>
                <Transition>
                    {pack_list_view}
                </Transition>
            </ul>
            <AddInput on_add=add_pack/>
        </Show>
    }
}

#[component]
fn Pack() -> impl IntoView {
    let params = params::use_pack_params();

    let name = move || {
        params
            .with(|params| params.pack().map(str::to_string))
            .expect(":pack_name")
    };

    let selected_card = move || params.with(|params| params.card().map(str::to_string));

    let save_action = context::SaveAction::use_context().unwrap();

    let cards = create_resource(
        move || (name(), save_action.version().get()),
        |(name, _)| invoke::list_cards(name),
    );

    let card_list = move || {
        cards
            .get()
            .map(|cards| view! { <CardList cards selected_card/> })
    };

    view! {
        <a href="/">"Back"</a>
        <h1>{name}</h1>
        <A href="study">"Begin study"</A>
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

    let save_action = context::SaveAction::use_context().unwrap();

    let save = move |contents| {
        let params = params.get();
        let Some((pack_name, card_name)) = params.both() else {
            return;
        };

        save_action.dispatch((pack_name.to_string(), card_name.to_string(), contents));
    };

    let editor = move || {
        contents.get().map(|initial_contents| {
            view! {
                <Editor initial_contents on_save=save/>
            }
        })
    };

    view! {
        <h2>{card_name}</h2>
        <Transition>
            {editor}
        </Transition>
    }
}

#[component]
fn CardList(
    #[prop(into)] cards: MaybeSignal<BTreeSet<String>>,
    #[prop(into)] selected_card: Signal<Option<String>>,
) -> impl IntoView {
    let route = use_route();
    let path = route.path();
    let navigate = use_navigate();

    let add_card = move |card_name| {
        navigate(&format!("{path}/card/{card_name}"), Default::default());
    };

    let card_list_item = move |name: String| {
        let name = store_value(name);
        let is_selected = with!(|selected_card, name| selected_card.as_ref() == Some(name));

        view! {
            <li class:selected=is_selected>
                <A href=format!("card/{}", name.get_value())>
                    {name.get_value()}
                </A>
            </li>
        }
    };

    view! {
        <ul>
            <For
                each=move || cards.get()
                key=|card| card.clone()
                children=card_list_item
            />
        </ul>
        <AddInput on_add=add_card/>
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
fn Study() -> impl IntoView {
    let params = params::use_pack_params();
    let name = move || {
        params
            .with(|params| params.pack().map(str::to_string))
            .expect(":pack_name")
    };

    let card_contents = create_resource(name, invoke::deal_cards);

    view! {
        <h1>"Practicing " {name}</h1>
        <Transition>
            {move || card_contents.get().map(|card_contents| {
                view! {
                    <StudyCardSlides card_contents/>
                }
            })}
        </Transition>
    }
}

#[component]
fn StudyCardSlides(card_contents: Vec<String>) -> impl IntoView {
    use crate::slides::ThematicBreaks;

    let (card_index, set_card_index) = create_signal(0);

    let card_count = card_contents.len();

    let next_card = move |_| set_card_index.update(|i| *i += 1);

    let cards = card_contents
        .into_iter()
        .map(|contents| ThematicBreaks::new(&contents).map(str::to_string).collect())
        .collect::<Vec<_>>();

    let current_card = move || cards.get(card_index.get()).cloned();

    move || {
        if let Some(card_sections) = current_card() {
            view! {
                <h2>{move || card_index.get() + 1} "/" {card_count}</h2>
                <CardSlides card_sections on_complete=next_card/>
            }
            .into_view()
        } else {
            view! {
                <p>"You're done!"</p>
                <A href="/">"Return to packs"</A>
            }
            .into_view()
        }
    }
}

#[component]
fn CardSlides(
    card_sections: Vec<String>,
    #[prop(into)] on_complete: Callback<()>,
) -> impl IntoView {
    let (visible_count, set_visible_count) = create_signal(1);
    let section_count = card_sections.len();

    let next_section = move || set_visible_count.update(|count| *count += 1);

    let visible_sections = move || {
        card_sections
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
            {visible_sections}
            <Show
                when=move || visible_count.get() < section_count
                fallback=move || view! {
                    <li>
                        <button on:click=move |_| on_complete.call(())>
                            "Finish Card"
                        </button>
                    </li>
                }
            >
                <li>
                    <button on:click=move |_| next_section()>
                        "Next"
                    </button>
                </li>
            </Show>
        </ul>
    }
}
