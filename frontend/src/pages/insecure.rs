use crate::components::AudioPlayer;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Insecure)]
pub fn insecure() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"💗 Aww Are You Feeling A Lil Insecure?"}
            </h1>

            <img
                src="/static/cry.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <div>
                <AudioPlayer src={"/static/insecure.mp3".to_string()} />
            </div>

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Repeat after me: I am worthy, I am loved, I am amazing (because you are!) Listen to what I have to say, and if you still need to talk, please don't feel bad about reaching out to me ok? 😘"}
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
