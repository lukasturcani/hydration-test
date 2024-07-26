#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    #[cfg(not(feature = "prebuild"))]
    {
        // Init logger
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        info!("starting app");
        launch(App);
    }
    #[cfg(feature = "prebuild")]
    {
        let index_html = std::fs::read_to_string("dist/index.html").unwrap();
        let mut renderer = dioxus::ssr::Renderer::new();
        renderer.pre_render = true;
        let mut vdom = VirtualDom::new(App);
        vdom.rebuild_in_place();
        index_html.replace("<-- REPLACE -->", dioxus::ssr::pre_render(vdom));
    }
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut value = use_signal(|| 0);
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        div { id: "links",
            a { target: "_blank", href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { target: "_blank", href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
        button {
            onclick: move |_| {
                value.set(value() + 1);
                info!("value: {}", value());
            },
            "click me"
        }
    }
}
