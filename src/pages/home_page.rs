use leptos::prelude::*;
use leptos::{component, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>Welcome to Leptos!</h1>
            <p>This is a simple example of a Leptos application.</p>
        </div>
    }
}
