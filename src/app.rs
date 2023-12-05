mod home;
mod settings;

use wasm_bindgen::prelude::*;
use yew_router::prelude::*;
use yew::prelude::*;
use home::Home;
use settings::Settings;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/settings")]
    Settings,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Settings => html! {
            <Settings />
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}