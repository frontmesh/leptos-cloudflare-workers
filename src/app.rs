use leptos::prelude::*;
use leptos::{component, config::LeptosOptions, IntoView};
use leptos_meta::{provide_meta_context, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::HomePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Leptos Cloudflare implementation"/>
         <Meta name="description" content="Showcasing Leptos Cloudflare implementation"/>
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage />
            </Routes>
        </Router>
    }
}
