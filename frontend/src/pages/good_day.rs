use crate::components::confetti::ConfettiButton;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(GoodDay)]
pub fn good_day() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"Yayyy Let's Goooo 🎉 My Sonu Had A Good Day ☀️"}
            </h1>

            <img
                src="/static/good_day.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"I'm so happy to hear that you had a good day, Sonu! 🌈✨ Remember, every good day is a reason to celebrate! Keep shining and spreading your positivity. You deserve all the happiness in the world! 💖🌟"}
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
