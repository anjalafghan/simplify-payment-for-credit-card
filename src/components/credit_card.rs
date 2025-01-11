use dioxus::prelude::*;

#[component]
pub fn CreditCard() -> Element {
    rsx! {
        div {
            id: "credit-card",
            h1 { "Credit Card" }
            p { "This is the credit card page. component" }
        }
    }
}
