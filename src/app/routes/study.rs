use leptos::*;
use leptos_router::*;

use crate::{invoke, params};

// "/pack/:pack_name/study"
#[component]
pub fn StudyPage() -> impl IntoView {
    let name = params::use_pack_name();

    let card_contents = create_resource(move || name.get(), invoke::deal_cards);

    view! {
        <header>
            <h1>"Practicing " {name}</h1>
        </header>
        <main>
            <Transition>
                {move || card_contents.get().map(|card_contents| {
                    view! {
                        <StudyCardSlides card_contents/>
                    }
                })}
            </Transition>
        </main>
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
