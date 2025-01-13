use dioxus::prelude::*;

use crate::backend::save_transaction;

#[component]
pub fn CreditCard(id: usize, name: String) -> Element {
    let data = use_signal(|| String::new());
    rsx! {
        div {
            id: "credit-card",
            h1 { "Credit Card {name}" },
            p{ " Some data"}


        }
    }
}
