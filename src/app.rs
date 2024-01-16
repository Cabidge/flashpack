use leptos::*;

use crate::commands::invoke;

#[component]
pub fn App() -> impl IntoView {
    let open_collection_action = create_action(|_: &()| async {
        invoke::<Option<String>>("open_collection", &())
            .await
            .unwrap()
    });

    let card_slides = vec![
        String::from("Hello"),
        String::from("Foo"),
        String::from("Bar"),
    ];

    let title = move || {
        open_collection_action.value().get().map(|name| {
            view! {
                <h1>{name}</h1>
            }
        })
    };

    view! {
        <main>
            {title}
            <button on:click=move |_| open_collection_action.dispatch(())>
                "Foo"
            </button>
            <Card card_slides/>
        </main>
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
