// src/pages/self_doubt.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(SelfDoubt)]
pub fn self_doubt() -> Html {
    html! {
        <div class="p-6 text-center space-y-4">
            <h1 class="text-3xl font-bold">{"Doubting Yourself? 🫶"}</h1>
            <p>{"Remember your strength and kindness are why I adore you. You’re amazing."}</p>
            <Link<Route> to={Route::Home} classes="bg-pink-500 text-white px-4 py-2 rounded hover:bg-pink-600">{"← Back to Home"}</Link<Route>>
        </div>
    }
}
