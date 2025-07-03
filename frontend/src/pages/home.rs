use crate::components::confetti::ConfettiButton;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let buttons = vec![
        (Route::Headache, "🤕 Headache"),
        (Route::Hug, "🤗 Hug"),
        (Route::Missing, "🥺 Missing"),
        (Route::Sad, "😢 Sad"),
        (Route::Overwhelmed, "💥 Overwhelmed"),
        (Route::Cuddle, "🧸 Cuddle"),
        (Route::Sleepy, "😴 Sleepy"),
        (Route::Text, "📩 Text"),
        (Route::SelfDoubt, "😔 Self-Doubt"),
        (Route::Story, "📖 Story"),
        (Route::Distraction, "🎮 Distraction"),
        (Route::GoodDay, "😊 Good Day"),
        (Route::BadDay, "💔 Bad Day"),
        (Route::Anxious, "😰 Anxious"),
        (Route::Lonely, "💭 Lonely"),
        (Route::Bored, "🥱 Bored"),
        (Route::Insecure, "😟 Insecure"),
        (Route::Confused, "🤯 Confused"),
        (Route::Rainy, "🌧️ Rainy"),
        (Route::Love, "❤️ Love"),
    ];

    html! {
        <div class="min-h-screen flex flex-col items-center justify-center px-16 py-8">
            <h1 class="text-4xl font-bold text-pink-700 mb-10 text-center font-sans">
                {"Hi Sonu 💖, how ya feelin?"}
            </h1>
            <h2 class="text-1xl font-bold text-pink-700 mb-8 text-center font-sans">
                {"Click on a button based on how you're feeling right now or what you need right now 🩷 or even some confetti 🎉"}
            </h2>

            <ConfettiButton />

            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
                { for buttons.iter().map(|(route, label)| html! {
                    <div class="flex justify-center">
                        <Link<Route> to={route.clone()}>
                            <button class="w-44 bg-pink-200 border-pink-400 border-4 rounded-full px-4 py-2 font-semibold text-pink-700 hover:bg-pink-300 transition-transform transform hover:scale-105 active:scale-105 text-sm sm:text-base">
                                { label }
                            </button>
                        </Link<Route>>
                    </div>
                })}
            </div>
        </div>
    }
}
