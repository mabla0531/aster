use std::collections::HashMap;

use crate::{
    database,
    model::TransactionStatus,
};

pub async fn handle_cash(tx_id: u32, cash_amount: u32, items: HashMap<u32, u32>) -> Result<TransactionStatus, String> {
    match database::get_prices(items.keys().cloned().collect()).await {
        Ok(prices) => {
            let total = items.iter().map(|(id, qty)| prices.get(id).unwrap_or(&0) * qty).sum::<u32>();
            let difference: i32 = total as i32 - cash_amount as i32;
            if difference > 0 {
                match database::create_partial_transaction(tx_id, items.clone(), difference as u32).await {
                    Ok(_) => Ok(TransactionStatus::Partial {
                        remaining: difference as u32,
                    }),
                    Err(e) => {
                        eprintln!("Error creating partial transaction: {}", e);
                        Err(format!("Error creating partial transaction: {}", e))
                    }
                }
            } else { // if it's zero or cashback
                if let Err(e) = database::log_transaction(tx_id, items, difference.abs() as u32).await {
                    eprintln!("Non-critical logging error, still sending completion: {}", e);
                }
                Ok(TransactionStatus::Success {
                    cash_back: difference.abs() as u32,
                })
            }
        }
        Err(e) => Err(format!("Error getting total for items: {}", e.to_string()))
    }
}

pub async fn handle_credit(tx_id: u32, account_id: u32, items: HashMap<u32, u32>) -> Result<TransactionStatus, String> {
    match database::get_account(account_id).await {
        Ok(account) => match database::get_prices(items.keys().cloned().collect()).await {
            Ok(prices) => {
                let total = items.iter().map(|(id, qty)| prices.get(id).unwrap_or(&0) * qty).sum::<u32>();
                
                //                     e.g. 9000  - (9000  * (       10        / 100)) = 9000 - 900 = 8100
                let discounted_total = total - (total * (account.discount / 100));

                // screen for insufficient credit
                if account.credit < discounted_total && account.overdraft == false {
                    return Ok(TransactionStatus::Failure { reason: "Insufficient Credit.".to_string() });
                }

                match database::deduct_balance(account_id, discounted_total).await {
                    Ok(_) => {
                        if let Err(e) = database::log_transaction(tx_id, items, 0).await {
                            eprintln!("Non-critical logging error, continuing: {}", e);
                        }
                 
                        Ok(TransactionStatus::Success { cash_back: 0 })
                    },
                    Err(e) => Err(format!("Error deducting balance from account with id {}: {}", account_id, e)),
                }
            }
            Err(e) => {
                Err(format!("Error querying items in database: {}", e))
            }
        },
        Err(e) => Err(format!("Error looking up account {}: {}", account_id, e)),
    }
}
