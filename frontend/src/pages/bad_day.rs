use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(BadDay)]
pub fn bad_day() -> Html {
    html! {
        <div class="min-h-screen bg-pink-100 flex flex-col items-center justify-center p-8">
            <h1 class="text-3xl font-bold text-pink-800 mb-6 text-center">
                {"I'm sorry you're having a rough day 💔"}
            </h1>
            <p class="text-center text-pink-700 mb-6 max-w-md">
                {"No matter what happened, I love you and I'm here for you. Breathe. You’re not alone."}
            </p>
            <Link<Route> to={Route::Home}>
                <button class="bg-pink-200 border-pink-400 border-4 rounded-full px-6 py-2 font-semibold text-pink-900 hover:bg-pink-300 transition-transform transform hover:scale-105 active:scale-105">
                    {"← Back Home"}
                </button>
            </Link<Route>>
        </div>
    }
}
