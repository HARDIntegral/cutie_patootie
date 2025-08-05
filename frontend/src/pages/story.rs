use crate::components::confetti::ConfettiButton;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(Story)]
pub fn story() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-center px-4 py-12">
            <h1 class="text-3xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"Story Time It Is Then 📖"}
            </h1>

            <img
                src="/static/story.gif"
                alt="Cute comfort gif"
                class="w-64 h-auto rounded-lg mb-4"
            />

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Once upon a time, there was a pretty princess and a handsome prince. The prince saw the princess all the way from the floor, and when he looked into her eyes, he saw his world. Ever since that moment, he swore to be with his princess. He climbed mountains and defeated all the monsters in his path. He finally found his princess waiting for him. They lived happily ever after, and the prince never let his princess go. The End. 🤴🏽💞👸🏽"}
            </h2>

            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"What? That wasn't enough? It's ok, we're still writing that story ;)"}
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
