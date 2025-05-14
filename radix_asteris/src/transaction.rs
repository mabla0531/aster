use std::collections::HashMap;

use crate::database;
use model::TransactionStatus;

pub async fn calc_total(tx_id: String, items: HashMap<u32, u32>) -> Result<u32, String> {
    match database::check_partial_transaction(tx_id.clone()).await {
        Some(total) => Some(total), // I would love to not do this, but async closures cannot be passed to map_or
        None => {
            println!("No partial transaction found, calculating regular total");
            database::get_prices(items.keys().cloned().collect()).await.map(|prices| items
                .iter()
                .map(|(id, qty)| prices.get(id).unwrap_or(&0) * qty)
                .sum::<u32>()
            ).map_err(|e| println!("error calculating transactions: {}", e)).ok()
        }
    }.ok_or("Failed to look up total for transaction.".to_string())
}

pub async fn handle_cash(
    tx_id: String,
    cash_amount: u32,
    items: HashMap<u32, u32>,
    total: u32,
) -> Result<TransactionStatus, String> {
    let difference: i32 = total as i32 - cash_amount as i32;
    if difference > 0 {
        match database::create_partial_transaction(tx_id.clone(), items.clone(), difference as u32)
            .await
        {
            Ok(_) => Ok(TransactionStatus::Partial {
                remaining: difference as u32,
            }),
            Err(e) => {
                eprintln!("Error creating partial transaction: {}", e);
                Err(format!("Error creating partial transaction: {}", e))
            }
        }
    } else {
        // if it's zero or cashback
        if let Err(e) = database::log_transaction(tx_id.clone(), items, difference.unsigned_abs()).await {
            eprintln!("Non-critical logging error, still sending completion: {}", e);
        }
        if let Err(e) = database::drop_partial_transaction(tx_id).await {
            eprintln!("Error dropping partial tx that may or may not exist, continuing regardless: {}", e);
        }
        Ok(TransactionStatus::Success {
            cash_back: difference.unsigned_abs(),
        })
    }
}

pub async fn handle_credit(
    tx_id: String,
    account_id: u32,
    items: HashMap<u32, u32>,
    total: u32,
) -> Result<TransactionStatus, String> {
    match database::get_account(account_id).await {
        Ok(account) => {
            //                     e.g. 9000  - (9000  * (       10        / 100)) = 9000 - 900 = 8100
            let discounted_total = total - (total * (account.discount / 100));

            // screen for insufficient credit
            if account.credit < discounted_total && !account.overdraft {
                return Ok(TransactionStatus::Failure {
                    reason: "Insufficient Credit.".to_string(),
                });
            }

            match database::deduct_balance(account_id, discounted_total).await {
                Ok(_) => {
                    if let Err(e) = database::log_transaction(tx_id.clone(), items, 0).await {
                        eprintln!("Non-critical logging error, continuing: {}", e);
                    }

                    if let Err(e) = database::drop_partial_transaction(tx_id).await {
                        eprintln!("Error dropping partial tx that may or may not exist, continuing regardless: {}", e);
                    }

                    Ok(TransactionStatus::Success { cash_back: 0 })
                }
                Err(e) => Err(format!(
                    "Error deducting balance from account with id {}: {}",
                    account_id, e
                )),
            }
        },
        Err(e) => Err(format!("Error looking up account {}: {}", account_id, e)),
    }
}
