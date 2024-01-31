use leptos::*;
use leptos_router::*;

use crate::{context, invoke};

// "/"
#[component]
pub fn RootPage() -> impl IntoView {
    let save_action = context::SaveAction::use_context().unwrap();

    let open_collection_action = create_action(|_: &()| invoke::open_collection());

    let collection_name = create_resource(
        move || open_collection_action.version().get(),
        |_| invoke::get_collection_name(),
    );

    let title = move || collection_name.get().flatten();

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
                            <A class="button" href>{pack_name}</A>
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
        <header>
            <button on:click=move |_| open_collection_action.dispatch(())>
                "Open Collection"
            </button>
            <h1>{title}</h1>
        </header>
        <main>
            <Show
                when=move || collection_name.with(|name| matches!(name, Some(Some(_))))
                fallback=|| view! { <h1>"No Collection Selected..."</h1> }
            >
                <h2>"Packs"</h2>
                <ul class="pack-list">
                    <Transition>
                        {pack_list_view}
                    </Transition>
                    <li class="pack-list-item">
                        <AddButton on_add=add_pack/>
                    </li>
                </ul>
            </Show>
        </main>
    }
}

#[component]
fn AddButton(#[prop(into)] on_add: Callback<String>) -> impl IntoView {
    let (value, set_value) = create_signal(None::<String>);

    let is_active = move || value.with(Option::is_some);

    let inactive_button = move || {
        view! {
            <button
                class="ghost-add"
                on:click=move |_| set_value.set(Some(String::new()))
            >
                "Add"
            </button>
        }
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let pack_name = set_value
            .try_update(|value| value.take())
            .flatten()
            .unwrap_or_default();

        on_add.call(pack_name);
    };

    let on_unfocus = move |_| {
        set_value.set(None);
    };

    let on_input = move |ev| {
        let new_value = event_target_value(&ev);
        set_value.set(Some(new_value));
    };

    // auto focus
    let input_ref = create_node_ref::<html::Input>();
    create_effect(move |_| {
        let Some(input) = input_ref.get() else {
            return;
        };

        let _ = input.on_mount(|input| {
            let _ = input.focus().ok();
        });
    });

    view! {
        <Show
            when=is_active
            fallback=inactive_button
        >
            <form class="ghost-add active" on:submit=on_submit>
                <input
                    node_ref=input_ref
                    placeholder="New Pack"
                    on:input=on_input
                    on:focusout=on_unfocus
                />
            </form>
        </Show>
    }
}
