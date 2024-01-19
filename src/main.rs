mod app;
mod commands;
mod context;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
