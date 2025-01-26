use dioxus::prelude::*;

use crate::backend::{delete_card, list_cards_for_deletion};

#[component]
pub fn DeleteCard() -> Element {
    let list_cards = use_server_future(move || list_cards_for_deletion())?;
    let cards = list_cards.unwrap().unwrap();

    rsx! {
      div {
        id: "create-card",
        class: "card-container",

        h1 {
          class: "card-title",
          "Credit Card Deletion"
        }
        form {
          class: "card-form",
          onsubmit: move |event| async move {
            // Extract values from the form
            if let (Some(id), ) = (
                event.values().get("id").and_then(|v| v.get(0)).and_then(|id| Some(id.parse::<usize>())),
            ) {
              tracing::info!("Received values - id: {:?}", id);

              match delete_card(id.expect("Conversion failed")).await {
                Ok(_) => {
                  tracing::info!("Card deleted successfully");
                }
                Err(e) => {
                  tracing::error!("Error saving card: {:?}", e);
                }
              }
            }
          },
          div {
            class: "input-group",
            label {
              class: "input-label",
              "Select Card"
            }
            select {
              name: "id",
              class: "input-field",
              required: true,
              for i in cards {
                option {
                  value: "{i.id}",
                  "name: {i.name}"
                }
              }
            }
          }
          input {
            r#type: "submit",
            class: "submit-btn",
            value: "Delete Card Button"
          }
        }
      }
    }
}
