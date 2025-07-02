use wasm_bindgen::prelude::*; // gives #[wasm_bindgen(start)]
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod routes; // src/routes.rs // src/pages/mod.rs re-exports every page

use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)] // single entry-point for the WASM lib
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
