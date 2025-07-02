// src/pages/sleepy.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Sleepy)]
pub fn sleepy() -> Html {
    html! {
        <div class="p-6 text-center space-y-4">
            <h1 class="text-3xl font-bold">{"Can’t Sleep? 💤"}</h1>
            <p>{"Count tiny sheep, breathe slowly, and picture us watching the stars. Goodnight, love."}</p>
            <Link<Route> to={Route::Home} classes="bg-pink-500 text-white px-4 py-2 rounded hover:bg-pink-600">{"← Back to Home"}</Link<Route>>
        </div>
    }
}
