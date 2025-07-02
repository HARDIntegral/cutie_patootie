// src/pages/missing.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Missing)]
pub fn missing() -> Html {
    html! {
        <div class="p-6 text-center space-y-4">
            <h1 class="text-3xl font-bold">{"Missing Me? 💌"}</h1>
            <p>{"I miss you too. Hold this page close like a note in your pocket until I’m with you."}</p>
            <Link<Route> to={Route::Home} classes="bg-pink-500 text-white px-4 py-2 rounded hover:bg-pink-600">{"← Back to Home"}</Link<Route>>
        </div>
    }
}
