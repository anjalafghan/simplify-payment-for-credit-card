use components::Navbar;
use dioxus::prelude::*;
use tracing_subscriber;
use views::{CreateCardView, DeleteCardView, Home};
mod backend;
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/create-card")]
    CreateCardView {},
    #[route("/delete-card")]
    DeleteCardView {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CREDIT_CSS: Asset = asset!("/assets/styling/credit_card.css");
const CREATE_CARD: Asset = asset!("/assets/styling/create_card.css");

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("info");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    rsx! {

        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: CREDIT_CSS}
        document::Link { rel: "stylesheet", href: CREATE_CARD}


        Router::<Route> {}
    }
}
