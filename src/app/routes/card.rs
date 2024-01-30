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
    let contents = create_rw_signal(initial_contents);
    let (saved_contents, set_saved_contents) = create_signal(contents.get_untracked());

    let save_action = context::SaveAction::use_context().unwrap();

    let is_edited = move || with!(|saved_contents, contents| saved_contents != contents);

    let on_reset = move |_| {
        contents.set(saved_contents.get());
    };

    let on_save = move |_| {
        save_action.dispatch((pack_name.get(), card_name.get(), contents.get()));
        set_saved_contents.set(contents.get());
    };

    view! {
        <Editor contents/>
        <Show when=is_edited>
            <div class="editor-button-group">
                <button on:click=on_reset>
                    "Discard"
                </button>
                <button class="primary" on:click=on_save>
                    "Save"
                </button>
            </div>
        </Show>
    }
}

/// A wrapper for AutosizeTextarea that has custom styling.
#[component]
fn Editor(contents: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="editor">
            <AutosizeTextarea value=contents/>
        </div>
    }
}

/// A textarea that automatically changes its height based on its contents.
#[component]
fn AutosizeTextarea(value: RwSignal<String>) -> impl IntoView {
    let on_input = move |ev| {
        let new_value = event_target_value(&ev);
        value.set(new_value);
    };

    let textarea_ref = create_node_ref::<html::Textarea>();

    create_effect(move |_| {
        value.track();

        if let Some(node_ref) = textarea_ref.get() {
            let node_ref = node_ref.style("height", "auto");
            let new_height = format!("{}px", node_ref.scroll_height());
            let _ = node_ref.style("height", new_height);
        }
    });

    view! {
        <textarea
            node_ref=textarea_ref
            prop:value=move || value.get()
            on:input=on_input
            rows="1"
        >
            {value.get_untracked()}
        </textarea>
    }
}
