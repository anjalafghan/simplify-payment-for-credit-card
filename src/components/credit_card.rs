use crate::backend::get_transactions;
use crate::backend::save_transaction;
use dioxus::prelude::*;

#[component]
pub fn CreditCard(id: usize, name: String) -> Element {
    let mut data = use_signal(|| 0.0);
    rsx! {
    div {
        id: "credit-card",
        h1 { "Credit Card {name}" },
        button {
                onclick: move |_| async move  {
                    match get_transactions(id).await{
                    Ok(amount) =>{
                        data.set(amount);
                    }
                    Err(e) => {
                        tracing::info!("Not able to fetch the details {}", e);
                    }
                }
                } ,
        "Click to view Data"
        },
        p{ "{data}"}

        div{
            form { onsubmit: move |event| async move {
                if let Some(amount) = event.values().get("current_card_amount").and_then(|value| value.get(0)).and_then(|s| s.parse::<f64>().ok()) {
                    data.set(amount);
                    tracing::info!("Amount captured {:?}", amount);
                    match save_transaction(id, amount).await{
                        Ok(_) => {
                            tracing::info!("Transaction saved successfully");
                        }
                        Err(e) => {
                            tracing::error!("Error saving transaction: {:?}", e);
                        }
                    }
                }
            },
                        input { name: "current_card_amount" }
                        input { r#type: "submit" }
                    }
        }
        }
    }
}

//Ideally the save transaction should save the difference and the show transactions should show all payments made on the card
