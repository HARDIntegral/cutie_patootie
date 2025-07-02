// src/pages/good_day.rs
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(GoodDay)]
pub fn good_day() -> Html {
    html! {
        <div class="p-6 text-center space-y-4">
            <h1 class="text-3xl font-bold">{"Had a Good Day? ☀️"}</h1>
            <p>{"Yay! I’m so proud. Let’s celebrate with an imaginary confetti blast! 🎉"}</p>
            <Link<Route> to={Route::Home} classes="bg-pink-500 text-white px-4 py-2 rounded hover:bg-pink-600">{"← Back to Home"}</Link<Route>>
        </div>
    }
}
