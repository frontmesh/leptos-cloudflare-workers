#[cfg(feature = "ssr")]
use worker::*;

#[allow(unused_imports)]
use crate::app::*;

pub mod app;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod worker_helpers;

#[cfg(feature = "ssr")]
async fn router(_env: Env) -> axum::Router {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // build our application with a route
    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .with_state(leptos_options)
}

#[cfg(feature = "ssr")]
#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    use tower_service::Service;

    Ok(router(env).await.call(req).await?)
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
}
