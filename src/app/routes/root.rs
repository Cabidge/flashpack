use leptos::*;
use leptos_router::*;

use crate::{app::AddInput, context, invoke};

// "/"
#[component]
pub fn RootPage() -> impl IntoView {
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
        <header>
            <h1>{title}</h1>
            <button on:click=move |_| open_collection_action.dispatch(())>
                "Open Collection"
            </button>
        </header>
        <main>
            <Show when=move || collection_name.with(|name| matches!(name, Some(Some(_))))>
                <h2>"Packs"</h2>
                <ul class="pack-list">
                    <Transition>
                        {pack_list_view}
                    </Transition>
                </ul>
                <AddInput on_add=add_pack/>
            </Show>
        </main>
    }
}
