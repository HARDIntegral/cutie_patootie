use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Bored)]
pub fn bored() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"🎲 You Bored???"}
            </h1>

            <img
                src="/static/bored.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Send me an iMessage game, or better yet get into your closet and play dress up, I wanna see how cute you look when you're bored 🥰"}
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
