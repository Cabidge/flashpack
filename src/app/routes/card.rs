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
        <Editor initial_contents={contents.get_untracked()} set_contents/>
        <Show when=move || is_edited.get()>
            <button class="save-button" on:click=on_click>
                "Save"
            </button>
        </Show>
    }
}

#[component]
fn Editor(initial_contents: String, set_contents: WriteSignal<String>) -> impl IntoView {
    let on_input = move |ev| {
        let target = event_target::<web_sys::HtmlDivElement>(&ev);
        set_contents.set(target.inner_text());
    };

    view! {
        <div class="editor">
            <div contenteditable on:input=on_input>
                {initial_contents}
            </div>
        </div>
    }
}
