use dioxus::prelude::*;

#[component]
pub fn CreditCard(name: String) -> Element {
    rsx! {
        div {
            id: "credit-card",
            h1 { "Credit Card {name}" }
            p { "This is the credit card page. component" }
        }
    }
}
