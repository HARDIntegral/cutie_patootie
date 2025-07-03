use crate::components::confetti::ConfettiButton;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Hug)]
pub fn hug() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"Does Sonu Need A Virtual Hug 🤗"}
            </h1>

            <img
                src="/static/hug.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Sending you virtual hugs and cuddles! 🤗💖 Imagine me wrapping my arms around you, giving you the warmest hug ever. You're not alone, Sonu! I'm here for you, always 🥰"}
            </h2>

            <ConfettiButton />

            <Link<Route>
                to={Route::Home}
                classes="bg-pink-500 text-white px-6 py-2 rounded border-2 border-pink-600 hover:scale-105 transition transform duration-200"
            >
                {"← Back to Home"}
            </Link<Route>>
        </div>
    }
}
