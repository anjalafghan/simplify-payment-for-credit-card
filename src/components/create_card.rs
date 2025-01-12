use dioxus::prelude::*;

use crate::backend::save_card;

#[component]
pub fn CreateCard() -> Element {
    rsx! {
        div {
            id: "create-card",
            h1 { "Credit Card creation" }
            form { onsubmit: move |event| async move {
                if let Some(name) = event.values().get("name").and_then(|value| value.get(0)) {
                    tracing::info!("First value is {:?}", name);
                    match save_card(name.to_string()).await{
                        Ok(_) => {
                            tracing::info!("Card saved successfully");
                        }
                        Err(e) => {
                            tracing::error!("Error saving card: {:?}", e);
                        }
                    }
                }
            },
                        input { name: "name" }
                        input { r#type: "submit" }
                    }
        }
    }
}
