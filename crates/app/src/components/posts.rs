use dioxus::prelude::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

pub fn Posts(cx: Scope) -> Element {
    let mut current_post = use_state(&cx, || 1);

    let fut = use_future(&cx, (current_post,), |(current_post,)| async move {
        reqwest::get(format!(
            "https://jsonplaceholder.typicode.com/posts/{}",
            *current_post
        ))
        .await
        .unwrap()
        .json::<Post>()
        .await
    });

    cx.render(match fut.value() {
        Some(Ok(resp)) => rsx! {
            button {
                onclick: move |_| {
                  current_post += 1;
                  fut.restart()
                },
                "Next post"
            }
            div {
                h2 { "{resp.title}" }
                p { "{resp.body}" }
            }
        },
        Some(Err(_)) => rsx! { div { "loading post failed" } },
        None => rsx! { div { "loading post..." } },
    })
}
