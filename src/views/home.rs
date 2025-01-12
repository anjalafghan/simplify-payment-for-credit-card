use crate::{backend::list_cards, components::CreditCard, App};
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Home() -> Element {
    let cards = use_resource(move || async move { list_cards().await });

    rsx! {


        CreditCard { name: "Anjal" }
    }
}
