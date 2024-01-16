use leptos::*;
use serde::Serialize;

use crate::commands::invoke;

#[component]
pub fn App() -> impl IntoView {
    #[derive(Serialize)]
    struct Args {
        name: String,
    }

    let greetings = ["foo", "bar", "baz"]
        .into_iter()
        .map(|x| {
            let greeting = create_resource(
                || (),
                move |_| {
                    let name = String::from(x);
                    let args = Args { name };
                    async move { invoke::<String>("greet", &args).await.unwrap() }
                },
            );

            view! {
                <p>{move || greeting.get()}</p>
            }
        })
        .collect_view();

    view! {
        <main>
            {greetings}
        </main>
    }
}
