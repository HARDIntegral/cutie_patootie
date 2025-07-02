use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    Anxious, BadDay, Bored, Butterflies, Confused, Cuddle, Distraction, GoodDay, Headache, Home,
    Hug, Insecure, Lonely, Love, Missing, Overwhelmed, Rainy, Sad, SelfDoubt, Sleepy, Story, Text,
};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/headache")]
    Headache,
    #[at("/hug")]
    Hug,
    #[at("/missing")]
    Missing,
    #[at("/sad")]
    Sad,
    #[at("/overwhelmed")]
    Overwhelmed,
    #[at("/cuddle")]
    Cuddle,
    #[at("/sleepy")]
    Sleepy,
    #[at("/text")]
    Text,
    #[at("/self-doubt")]
    SelfDoubt,
    #[at("/story")]
    Story,
    #[at("/distraction")]
    Distraction,
    #[at("/good-day")]
    GoodDay,
    #[at("/bad-day")]
    BadDay,
    #[at("/anxious")]
    Anxious,
    #[at("/lonely")]
    Lonely,
    #[at("/bored")]
    Bored,
    #[at("/insecure")]
    Insecure,
    #[at("/confused")]
    Confused,
    #[at("/butterflies")]
    Butterflies,
    #[at("/rainy")]
    Rainy,
    #[at("/love")]
    Love,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Headache => html! { <Headache /> },
        Route::Hug => html! { <Hug /> },
        Route::Missing => html! { <Missing /> },
        Route::Sad => html! { <Sad /> },
        Route::Overwhelmed => html! { <Overwhelmed /> },
        Route::Cuddle => html! { <Cuddle /> },
        Route::Sleepy => html! { <Sleepy /> },
        Route::Text => html! { <Text /> },
        Route::SelfDoubt => html! { <SelfDoubt /> },
        Route::Story => html! { <Story /> },
        Route::Distraction => html! { <Distraction /> },
        Route::GoodDay => html! { <GoodDay /> },
        Route::BadDay => html! { <BadDay /> },
        Route::Anxious => html! { <Anxious /> },
        Route::Lonely => html! { <Lonely /> },
        Route::Bored => html! { <Bored /> },
        Route::Insecure => html! { <Insecure /> },
        Route::Confused => html! { <Confused /> },
        Route::Butterflies => html! { <Butterflies /> },
        Route::Rainy => html! { <Rainy /> },
        Route::Love => html! { <Love /> },
    }
}
