use crate::{backend::list_cards, components::CreditCard, App};
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Home() -> Element {
    let list_cards = use_server_future(move || list_cards())?;
    let cards = list_cards.unwrap().unwrap();

    rsx! {
        div { id: "main-container",
        div {
            class: "card-container",
        for card in cards {
            CreditCard {id: card.0, name: card.1, color: card.2, secondary_color: card.3,  card_type: card.4}
            }
        }
        }
    }
}

// We need to fetch all credit cards reactively
// How? we just hit the backend and get the data and if there is a data we render that
// Get the list of data from backend
// initialize a signal vector
// and then use signal to update the data
