mod editor;

use editor::SectionsEditor;

use leptos::*;

use crate::{context, invoke, params};

// "/pack/:pack_name/card/:card_name"
#[component]
pub fn CardPage() -> impl IntoView {
    let pack_name = params::use_pack_name();
    let card_name = params::use_card_name();

    let contents = create_resource(
        move || (pack_name.get(), card_name.get()),
        |(pack_name, card_name)| async move {
            invoke::get_card(pack_name, card_name)
                .await
                .unwrap_or_default()
        },
    );

    let editor = move || {
        contents.get().map(|initial_contents| {
            view! {
                <CardEditor
                    pack_name
                    card_name
                    initial_contents
                />
            }
        })
    };

    view! {
        <Transition>
            {editor}
        </Transition>
    }
}

#[component]
fn CardEditor(
    #[prop(into)] pack_name: Signal<String>,
    #[prop(into)] card_name: Signal<String>,
    initial_contents: String,
) -> impl IntoView {
    let (contents, set_contents) = create_signal(initial_contents);

    let save_action = context::SaveAction::use_context().unwrap();

    let (is_editing, set_is_editing) = create_signal(false);

    let on_save = move |new_contents: String| {
        save_action.dispatch((pack_name.get(), card_name.get(), new_contents.clone()));
        set_contents.set(new_contents);
        set_is_editing.set(false);
    };

    let fallback = move || {
        view! {
            <pre>{contents}</pre>
            <div class="editor-button-group">
                <button on:click=move |_| set_is_editing.set(true)>
                    "Edit"
                </button>
            </div>
        }
    };

    view! {
        <Show
            when=move || is_editing.get()
            fallback
        >
            <SectionsEditor
                initial_contents={contents.get_untracked()}
                on_discard=move |_| set_is_editing.set(false)
                on_save
            />
        </Show>
    }
}
