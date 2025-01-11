use crate::backend::save_dog;
use crate::components::CreditCard as OtherCreditCard;
use dioxus::prelude::*;
#[component]
pub fn CreditCard() -> Element {
    rsx! {

        OtherCreditCard {  }

    }
}
