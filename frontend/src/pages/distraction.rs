use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Distraction)]
pub fn distraction() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"🥺 Aww You Need A Lil Distraction?"}
            </h1>

            <img
                src="/static/distraction.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Hold on, lemme distract you with a couple of daddy jokes ;) Are you a loan? Because you sure have my interest! Even in zero gravity, I’d fall for you! Can I follow you home? My parents always told me to follow my dreams!"}
            </h2>

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"What? These didn't work? At this point just call me and I can give you plenty of things to think about to ease your mind 🥹"}
            </h2>

            <Link<Route>
                to={Route::Home}
                classes="bg-pink-500 text-white px-6 py-2 rounded border-2 border-pink-600 hover:scale-105 transition transform duration-200"
            >
                {"← Back to Home"}
            </Link<Route>>
        </div>
    }
}
