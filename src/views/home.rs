use crate::{backend::list_cards, components::CreditCard};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let list_cards = use_server_future(move || list_cards())?;
    let cards = list_cards.unwrap().unwrap();

    rsx! {
        div { id: "main-container",
        div {
            class: "card-container",
        for card in cards {
            CreditCard {id: card.0, name: card.1, color: card.2, secondary_color: card.3, button_color: card.4,  card_type: card.5}
            }
        }
        }
    }
}
