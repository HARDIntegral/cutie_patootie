use crate::components::AudioPlayer;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(SelfDoubt)]
pub fn self_doubt() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"Huhh? Why Are You Doubting Yourself? 🫶"}
            </h1>

            <img
                src="/static/doubt.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <div>
                <AudioPlayer src={"/static/doubt.mp3".to_string()} />
            </div>

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"You are the strongest most beautiful woman I know. I love you so much and I know you can do anything you set your mind to. Don't let self-doubt hold you back. Remember all the times you've overcome challenges and achieved great things. I'm always here to support you, so if you're feeling down, listen to what I have to say, and then call or text me and we can talk things out together 😘"}
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
