use dioxus::prelude::*;

use crate::backend::save_card;
use crate::components::color_generator;

#[component]
pub fn CreateCard() -> Element {
    rsx! {
        div {
            id: "create-card",
            class: "card-container",

            h1 { class: "card-title",
                "Credit Card Creation" }
            form {
                class: "card-form",
                onsubmit: move |event| async move {
                    // Extract values from the form
                    if let (Some(name), Some(color), Some(card_type)) = (
                        event.values().get("name").and_then(|v| v.get(0)),
                        event.values().get("color").and_then(|v| v.get(0)),
                        event.values().get("card_type").and_then(|v| v.get(0))
                    ) {
                        tracing::info!("Received values - Name: {:?}, Color: {:?}, Type: {:?}", name, color, card_type);
                        let (secondary_color, button_color) = color_generator(color);

                        match save_card(name.to_string(), color.to_string(), secondary_color.to_string(), button_color.to_string(),  card_type.to_string()).await {
                            Ok(_) => {
                                tracing::info!("Card saved successfully");
                            }
                            Err(e) => {
                                tracing::error!("Error saving card: {:?}", e);
                            }
                        }
                    }
                },
                div {
                                   class: "input-group",
                                   label { class: "input-label", "Name: " }
                                   input { name: "name", r#type: "text", class: "input-field", required: true }
                               }
                               div {
                                   class: "input-group",
                                   label { class: "input-label", "Card Color: " }
                                   input { name: "color", r#type: "color", class: "input-field", required: true }
                               }
                               div {
                                   class: "input-group",
                                   label { class: "input-label", "Card Type: " }
                                   select { name: "card_type", class: "input-field", required: true,
                                       option { value: "VISA", "VISA" }
                                       option { value: "MasterCard", "MasterCard" }
                                   }
                               }
                               input { r#type: "submit", class: "submit-btn", value: "Create Card" }
                           }
        }
    }
}
