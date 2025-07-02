// src/pages/overwhelmed.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Overwhelmed)]
pub fn overwhelmed() -> Html {
    html! {
        <div class="p-6 text-center space-y-4">
            <h1 class="text-3xl font-bold">{"Feeling Overwhelmed 🌧️"}</h1>
            <p>{"Breathe in… 4 seconds. Hold… 4. Out… 4. You’ve got this and I’ve got you."}</p>
            <Link<Route> to={Route::Home} classes="bg-pink-500 text-white px-4 py-2 rounded hover:bg-pink-600">{"← Back to Home"}</Link<Route>>
        </div>
    }
}
