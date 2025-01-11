use crate::components::CreditCard as OtherCreditCard;
use dioxus::prelude::*;
#[component]
pub fn CreditCard() -> Element {
    rsx! {

        OtherCreditCard {  }
        div {
            id: "credit-card",
            h1 { "Credit Card" }
            p { "This is the credit card page. view" }
        }
    }
}
