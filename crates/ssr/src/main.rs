use app::App;
use dioxus::prelude::*;
use rocket::{
    fs::{relative, FileServer},
    get, launch,
    response::{content, status},
    routes,
};
use std::fs;
use std::io;
use std::net::SocketAddr;

#[get("/")]
fn index() -> Result<content::RawHtml<String>, io::Error> {
    let mut vdom = VirtualDom::new(App);
    let _ = vdom.rebuild();
    let html = fs::read_to_string(relative!("../web/dist/index.html"))?;
    let content = dioxus::ssr::render_vdom(&vdom);

    Ok(content::RawHtml(html.replace("%html%", &content)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("../web/dist")))
}
