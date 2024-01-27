mod app;
mod context;
mod invoke;
mod params;

mod slides;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
