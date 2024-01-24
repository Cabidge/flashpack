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

    let (header, set_header) = create_signal(None::<web_sys::Element>);

    create_effect(move |_| {
        let header = document().get_element_by_id("header").expect("#header");
        set_header.set(Some(header));
    });

    context::Header::provide(header.into());

    view! {
        <header id="header"></header>
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
fn Header(children: ChildrenFn) -> impl IntoView {
    let header = context::Header::use_context().expect("Header context");

    let children = store_value(children);

    move || {
        header.get().map(|header| {
            view! {
                <Portal mount=header>
                    {children.get_value()()}
                </Portal>
            }
        })
    }
}

// "/"
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
                        <li class="pack-list-item">
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
        <Header>
            <h1>{title}</h1>
            <button on:click=move |_| open_collection_action.dispatch(())>
                "Open Collection"
            </button>
        </Header>
        <Show when=move || collection_name.with(|name| matches!(name, Some(Some(_))))>
            <h2>"Packs"</h2>
            <ul class="pack-list">
                <Transition>
                    {pack_list_view}
                </Transition>
            </ul>
            <AddInput on_add=add_pack/>
        </Show>
    }
}

// "/pack/:pack_name"
#[component]
fn Pack() -> impl IntoView {
    let name = params::use_pack_name();

    let save_action = context::SaveAction::use_context().unwrap();

    let cards = create_resource(
        move || (name.get(), save_action.version().get()),
        |(name, _)| invoke::list_cards(name),
    );

    let card_list = move || cards.get().map(|cards| view! { <CardList cards/> });

    view! {
        <Header>
            <a href="/">"Back"</a>
            <h1>{name}</h1>
        </Header>
        <A href="study">"Begin study"</A>
        <Transition>
            {card_list}
        </Transition>
        <Outlet/>
    }
}

// "/pack/:pack_name/card/:card_name"
#[component]
fn CardEditor() -> impl IntoView {
    let pack_name = params::use_pack_name();
    let card_name = params::use_card_name();

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
        move || (pack_name.get(), card_name.get()),
        |(pack_name, card_name)| async move {
            invoke::get_card(pack_name, card_name)
                .await
                .unwrap_or_default()
        },
    );

    let save_action = context::SaveAction::use_context().unwrap();

    let save = move |contents| {
        save_action.dispatch((pack_name.get(), card_name.get(), contents));
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

// "/pack/:pack_name/study"
#[component]
fn Study() -> impl IntoView {
    let name = params::use_pack_name();

    let card_contents = create_resource(move || name.get(), invoke::deal_cards);

    view! {
        <Header>
            <h1>"Practicing " {name}</h1>
        </Header>
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
