use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAudioElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AudioPlayerProps {
    pub src: String,
}

#[function_component(AudioPlayer)]
pub fn audio_player(props: &AudioPlayerProps) -> Html {
    let audio_ref = use_node_ref();
    let progress = use_state(|| 0.0);

    {
        let audio_ref = audio_ref.clone();
        let progress = progress.clone();

        use_effect_with((), move |_| {
            let maybe_listener = if let Some(audio) = audio_ref.cast::<HtmlAudioElement>() {
                let progress = progress.clone();
                let audio_clone = audio.clone();
                Some(EventListener::new(&audio, "timeupdate", move |_| {
                    let dur = audio_clone.duration();
                    if dur.is_finite() && dur > 0.0 {
                        let current = audio_clone.current_time();
                        progress.set(current / dur);
                    }
                }))
            } else {
                None
            };

            move || {
                drop(maybe_listener);
            }
        });
    }

    // Seek handler
    let audio_for_seek = audio_ref.clone();
    let seek = Callback::<InputEvent>::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            if let Some(audio) = audio_for_seek.cast::<HtmlAudioElement>() {
                let percent = input.value_as_number() / 100.0;
                let duration = audio.duration();
                if duration.is_finite() && duration > 0.0 {
                    audio.set_current_time(duration * percent);
                }
            }
        }
    });

    // Playback controls
    let audio_for_play = audio_ref.clone();
    let on_play = Callback::from(move |_| {
        if let Some(audio) = audio_for_play.cast::<HtmlAudioElement>() {
            let _ = audio.play();
        }
    });

    let audio_for_pause = audio_ref.clone();
    let on_pause = Callback::from(move |_| {
        if let Some(audio) = audio_for_pause.cast::<HtmlAudioElement>() {
            audio.pause().ok();
        }
    });

    let audio_for_restart = audio_ref.clone();
    let on_restart = Callback::from(move |_| {
        if let Some(audio) = audio_for_restart.cast::<HtmlAudioElement>() {
            audio.set_current_time(0.0);
            let _ = audio.play();
        }
    });

    let on_seek = {
        let audio_ref = audio_ref.clone();
        let progress = progress.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(audio) = audio_ref.cast::<HtmlAudioElement>() {
                if let Some(target) = event.target_dyn_into::<HtmlInputElement>() {
                    let value = target.value().parse::<f64>().unwrap_or(0.0) / 100.0;
                    audio.set_current_time(audio.duration() * value);
                    progress.set(value);
                }
            }
        })
    };
    let progress_percent = format!("{}%", (*progress * 100.0).round());
    let style = format!("--progress: {}", progress_percent);

    html! {
        <div class="bg-pink-100 border-4 border-pink-300 rounded-2xl p-3 max-w-xl w-full space-y-2 mb-6">
            <audio ref={audio_ref.clone()} src={props.src.clone()} />

            /* progress bar only */
            <div class="w-full h-2 bg-pink-200 rounded-full overflow-hidden">
                <div
                    class="h-full bg-pink-500 transition-[width] duration-200"
                    style={format!("width: {}%;", (*progress * 100.0).round())}
                />
            </div>

            <div class="flex justify-center gap-4">
                <button onclick={on_play}    class="bg-pink-500 text-1xl text-white px-4 py-2 rounded border-2 border-pink-600 hover:scale-105 transition">{"Play"}</button>
                <button onclick={on_pause}   class="bg-pink-500 text-1xl text-white px-4 py-2 rounded border-2 border-pink-600 hover:scale-105 transition">{"Pause"}</button>
                <button onclick={on_restart} class="bg-pink-500 text-1xl text-white px-4 py-2 rounded border-2 border-pink-600 hover:scale-105 transition">{"Restart"}</button>
            </div>
        </div>
    }
}
