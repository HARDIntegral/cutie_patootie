use crate::components::confetti::ConfettiButton;
use crate::components::AudioPlayer;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Cuddle)]
pub fn cuddle() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"😚 Come Here, It's Cuddle Time"}
            </h1>

            <img
                src="/static/cuddle.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <div>
                <AudioPlayer src={"/static/cuddle_song.mp3".to_string()} />
            </div>

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Ayy baby girl, come here and lay in my arms. Let me hold you tight and cuddle you while you sleep on my chest while we listen to our favorite cozy songs like this one 🤗"}
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
