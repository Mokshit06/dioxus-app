use app::App;
use dioxus::prelude::*;

fn main() {
    dioxus::web::launch_cfg(App, |config| {
        config.hydrate(true);

        config
    })
}
