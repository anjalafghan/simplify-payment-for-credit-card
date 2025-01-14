use crate::{backend::list_cards, components::CreditCard, App};
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Home() -> Element {
    let list_cards = use_server_future(move || list_cards())?;
    let my_vec = list_cards.unwrap().unwrap();

    rsx! {
        for (id, name) in my_vec {
            CreditCard {id: id, name: name}
        }

    }
}

// We need to fetch all credit cards reactively
// How? we just hit the backend and get the data and if there is a data we render that
// Get the list of data from backend
// initialize a signal vector
// and then use signal to update the data
