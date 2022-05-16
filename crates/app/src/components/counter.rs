use dioxus::prelude::*;

pub fn Counter(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx! {
      button { onclick: move |_| count -= 1,
        "-"
      }
      span { "{count}" }
      button { onclick: move |_| count += 1,
        "+"
      }
    })
}
