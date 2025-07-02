// src/pages/distraction.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Distraction)]
pub fn distraction() -> Html {
    html! {
        <div class="p-6 text-center space-y-4">
            <h1 class="text-3xl font-bold">{"Need a Distraction? 🌈"}</h1>
            <p>{"Knock-knock… Who’s there? Olive. Olive who? Olive you! (Bad joke, huge love)."}</p>
            <Link<Route> to={Route::Home} classes="bg-pink-500 text-white px-4 py-2 rounded hover:bg-pink-600">{"← Back to Home"}</Link<Route>>
        </div>
    }
}
