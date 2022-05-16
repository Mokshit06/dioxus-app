use dioxus::prelude::*;

mod components;

use components::{Counter, Posts};

pub fn App(cx: Scope) -> Element {
    let mut state = use_state(&cx, || 0);

    cx.render(rsx! {
        h1 { "Hello world" }
        Counter {}
        Posts {}
    })
}
