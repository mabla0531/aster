use std::collections::HashMap;

use dioxus::signals::{Signal, Writable};
use model::{Account, BalanceUpdate, UpdateOperation};

pub async fn add_balance(account: u32, amount: u32) -> bool {
    let balance_update = BalanceUpdate {
        id: account,
        amount,
        operation: UpdateOperation::Add
    };

    update_balance(balance_update).await
}

pub async fn remove_balance(account: u32, amount: u32) -> bool {
    let balance_update = BalanceUpdate {
        id: account,
        amount,
        operation: UpdateOperation::Sub
    };

    update_balance(balance_update).await
}

pub async fn update_balance(balance_update: BalanceUpdate) -> bool {
    match crate::CLIENT
        .post("http://localhost:5555/accounts/balance")
        .json(&balance_update)
        .send()
        .await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.json::<String>().await {
                        Ok(_) => return true,
                        Err(e) => tracing::error!("Error parsing balance update response: {:?}", e),
                    }
                } else if res.status() == 500 {
                    tracing::error!("Error code 500 returned for balance update request: {:?}", res.text().await);
                }
            },
            Err(e) => tracing::error!("Error sending balance update request: {:?}", e),
    }

    false
}
