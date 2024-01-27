mod routes;

use routes::*;

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
            <Routes>
                <Route path="" view=RootPage/>
                <Route path="/pack/:pack_name" view=PackPage>
                    <Route path="/" view=|| view! { <h2>"No Card Selected..."</h2> }/>
                    <Route path="/card/:card_name" view=CardPage/>
                </Route>
                // We don't want to show the CardList when in study mode
                <Route path="/pack/:pack_name/study" view=StudyPage/>
            </Routes>
        </Router>
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
