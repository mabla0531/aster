use crate::{
    database,
    model::{TxItem, TxStatus},
};

pub async fn handle_cash(tx_id: u32, cash_amount: u32, items: Vec<TxItem>) -> TxStatus {
    match database::get_total(items.clone()).await {
        Ok(items_total) => {
            let difference: i32 = items_total as i32 - cash_amount as i32;
            if difference > 0 {
                match database::create_partial_transaction(tx_id, items.clone(), difference as u32).await {
                    Ok(_) => TxStatus::Partial {
                        remaining: difference as u32,
                    },
                    Err(e) => {
                        eprintln!("Error creating partial transaction: {}", e);
                        TxStatus::Error {
                            message: format!("Error creating partial transaction: {}", e),
                        }
                    }
                }
            } else { // if it's zero or cashback
                if let Err(e) = database::log_transaction(tx_id, items, difference.abs() as u32).await {
                    eprintln!(
                        "Non-critical logging error, still sending completion: {}",
                        e
                    );
                }
                TxStatus::Complete {
                    cash_back: difference.abs() as u32,
                }
            }
        }
        Err(e) => {
            TxStatus::Error {
                message: format!("Error querying items in database: {}", e),
            }
        }
    }
}

pub async fn handle_credit(tx_id: u32, items: Vec<TxItem>, account_id: u32) -> TxStatus {
    //   - check if account has credit enabled
    //   - check if account is below credit limit
    //   - check if account allows overdraft

    match database::query_account(account_id).await {
        Ok(account) => match database::get_total(items.clone()).await {
            Ok(total) => {
                // TODO do account info stuff here
                match database::deduct_balance(account_id, total).await {
                    Ok(_) => {
                        if let Err(e) = database::log_transaction(tx_id, items, 0).await {
                            eprintln!("Non-critical logging error, continuing: {}", e);
                        }
                 
                        TxStatus::Complete {
                            cash_back: 0,
                        }
                    },
                    Err(e) => TxStatus::Error {
                        message: format!("Error deducting balance from account with id {}: {}", account_id, e),
                    }
                }
            }
            Err(e) => {
                TxStatus::Error {
                    message: format!("Error querying items in database: {}", e),
                }
            }
        },
        Err(e) => TxStatus::Error {
            message: format!("Error looking up account {}: {}", account_id, e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_handle_partial_cash() {
        let expected = TxStatus::Partial { remaining: 200 };
        let result = handle_cash(1234, 800, vec![TxItem { id: 1234, quantity: 2 }]).await;

        assert_eq!(expected, result);
    }

    #[tokio::test]
    pub async fn test_handle_full_zero_cash() {
        let expected = TxStatus::Complete { cash_back: 0 };
        let result = handle_cash(1234, 1000, vec![TxItem { id: 1234, quantity: 2 }]).await;

        assert_eq!(expected, result);
    }

    #[tokio::test]
    pub async fn test_handle_full_nonzero_cash() {
        let expected = TxStatus::Complete { cash_back: 200 };
        let result = handle_cash(1234, 1200, vec![TxItem { id: 1234, quantity: 2 }]).await;

        assert_eq!(expected, result);
    }

    #[tokio::test]
    pub async fn test_handle_credit() {
        let expected = TxStatus::Complete { cash_back: 0 };
        let result = handle_credit(1234, vec![TxItem { id: 1234, quantity: 2 }], 1111).await;
        
        assert_eq!(expected, result);
    }
}