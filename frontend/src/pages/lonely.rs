use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Lonely)]
pub fn lonely() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"🐣 You Feel Lonely? Don't Worry I'm Here"}
            </h1>

            <img
                src="/static/lonely.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Look at the night sky, where ever you are. Look at the big beautiful moon and the stars. Look at how they shine. Just know that I'm looking up at them with you. And I'm always here for you, Sonu. Whenever you feel lonely, just remember that I'm always with you in spirit. You can call or text me anytime, and I'll be there to listen and support you. You're never truly alone 😘"}
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
