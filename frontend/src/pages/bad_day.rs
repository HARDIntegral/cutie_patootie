use crate::components::AudioPlayer;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(BadDay)]
pub fn bad_day() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"Awww I'm Sorry You Had A Bad Day 💔"}
            </h1>

            <img
                src="/static/cry.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <div>
                <AudioPlayer src={"/static/bad_day.mp3".to_string()} />
            </div>

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"It's ok, no matter what happens, I love you and I'm here for you. You’re not alone. Sonu take a deep breath and listen to what I have to say 💖✨. Remember, it's okay to have bad days, and I'm here to support you through it"}
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
