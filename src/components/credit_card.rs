use crate::backend::{get_all_transactions, get_transactions, save_transaction, Transaction};
use dioxus::prelude::*;

#[component]
pub fn CreditCard(
    id: usize,
    name: String,
    color: String,
    secondary_color: String,
    button_color: String,
    card_type: String,
) -> Element {
    let mut data = use_signal(|| String::new());
    let mut show_modal = use_signal(|| false);
    let mut transactions = use_signal(Vec::<Transaction>::new);

    rsx! {
        div {

            class: "credit-card",
            style: format!("background: linear-gradient(135deg, {}, {});", color, secondary_color),

            div { class: "card-chip" },
            div { class: "card-number", "1234 5678 9012 3456" },
            div {
                class: "card-details",
                div { class: "card-holder", "{name}" }
                div { class: "expiry-date", "12/25" }
            }
            div { class: "card-amount", "Amount to pay {data}" }
            div { class: "card-logo", "{card_type}" }

            form {
                onsubmit: move |event| async move {
                    if let Some(amount_showing_up_on_card) = event.values().get("current_card_amount")
                        .and_then(|value| value.get(0))
                        .and_then(|s| s.parse::<f64>().ok())
                    {
                        match get_transactions(id).await {
                            Ok(sum_amount_from_database) => {
                                let money_to_pay = amount_showing_up_on_card - sum_amount_from_database;
                                data.set(money_to_pay.to_string());
                                match save_transaction(id, money_to_pay).await {
                                    Ok(_) => tracing::info!("Transaction saved successfully"),
                                    Err(e) => tracing::error!("Error saving transaction: {:?}", e),
                                }
                            },
                            Err(_) => tracing::info!("handle later"),
                        }
                    }
                },
                input {
                    name: "current_card_amount",
                    id: "credit_card_input",
                    r#type: "text",
                    placeholder: "Enter Amount",
                }
                input {
                    r#type: "submit",
                    id: "pay_button",
                    style: format!("color: {};", button_color),
                    value: "Pay",
                }
            }
            button {
                onclick: move |_| async move {
                    tracing::info!("Card clicked");
                    match get_all_transactions(id).await {
                        Ok(txns) => {
                            transactions.set(txns); // Store fetched transactions
                            show_modal.set(true);   // Show the modal
                        }
                        Err(e) => tracing::error!("Error fetching transactions: {:?}", e),
                    }

                },
                "View Transactions"
            }
        }

        // Modal Component
        if *show_modal.read() {
            div {
                class: "modal-overlay",
                onclick: move |_| show_modal.set(false), // Close modal when clicked outside
                div {
                    class: "modal-content",
                    h2 { "Transaction History" }
                    ul {
                        for transaction in transactions.read().iter() {
                            li {
                                "Amount: ₹{transaction.amount}, Date: {transaction.timestamp}"
                            }
                        }
                    }
                    button {
                        onclick: move |_| show_modal.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}
