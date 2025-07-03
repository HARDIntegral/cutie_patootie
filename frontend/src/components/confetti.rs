use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlAudioElement};
use yew::prelude::*;

/// Bind to the JS confetti function in `src/js/confetti_wrapper.js`
#[wasm_bindgen(module = "/src/js/confetti_wrapper.js")]
extern "C" {
    #[wasm_bindgen(js_name = fire_confetti)]
    fn fire_confetti_js();
}

fn play_pop_sound() {
    if let Some(win) = window() {
        if let Ok(audio) = HtmlAudioElement::new_with_src("/static/confetti.mp3") {
            audio.set_volume(0.7);
            let _ = audio.play();
        } else {
        }
    }
}

#[function_component(ConfettiButton)]
pub fn confetti_button() -> Html {
    let trigger = Callback::from(|_| {
        fire_confetti_js();
        play_pop_sound();
    });

    html! {
        <button
            onclick={trigger}
            class=" bg-pink-500 text-white px-6 py-2 rounded border-2 border-pink-600 hover:scale-105 transition transform duration-200 mb-6">
            {"🎊  More Confetti!"}
        </button>
    }
}
