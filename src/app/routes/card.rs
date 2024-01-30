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

    let save_action = context::SaveAction::use_context().unwrap();

    let reset_edit_status = create_trigger();

    let is_edited = create_memo(move |prev| {
        let was_edited = prev.cloned().unwrap_or(true);

        if was_edited {
            // if the previous state was true,
            // either this is the initial state or
            // the reset was triggered,
            // so we track for a change in contents
            // and set is_edited to false
            contents.track();
            false
        } else {
            // if the previous state was false,
            // that means we were tracking a change in contents,
            // and contents was changed, so we set is_edited
            // to true and track the reset trigger
            reset_edit_status.track();
            true
        }
    });

    let on_click = move |_| {
        reset_edit_status.notify();
        save_action.dispatch((pack_name.get(), card_name.get(), contents.get()));
    };

    view! {
        <Editor contents/>
        <Show when=move || is_edited.get()>
            <button class="save-button" on:click=on_click>
                "Save"
            </button>
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
