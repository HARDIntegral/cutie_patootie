// src/pages/anxious.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Anxious)]
pub fn anxious() -> Html {
    html! {
        <div class="p-6 text-center space-y-4">
            <h1 class="text-3xl font-bold">{"Feeling Anxious 🌬️"}</h1>
            <p>{"Let’s inhale calm and exhale worries together. I’m right beside you."}</p>
            <Link<Route> to={Route::Home} classes="bg-pink-500 text-white px-4 py-2 rounded hover:bg-pink-600">{"← Back to Home"}</Link<Route>>
        </div>
    }
}
