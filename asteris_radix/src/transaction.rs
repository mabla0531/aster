use crate::{database, model::{TxEntry, TxStatus}};

pub fn handle_cash(tx_id: u32, total: u32, items: Vec<TxEntry>) -> TxStatus {
    //   - nothing extra needed
    match database::query_items(items) {
        Ok(items) => {
            let items_total = items.iter().map(|item| item.price * item.quantity).sum::<u32>();
            let difference = total - items_total;
            if difference > 0 {
                match database::create_partial_transaction(tx_id, items, difference) {
                    Ok(_) => TxStatus::Partial { remaining: difference },
                    Err(e) => {
                        eprintln!("Error creating partial transaction: {}", e);
                        TxStatus::Error {
                            message: format!("Error creating partial transaction: {}", e),
                        }
                    }
                }
            } else {
                if let Err(e) = database::log_transaction(tx_id, items, difference) {
                    eprintln!("Non-critical logging error, still sending completion: {}", e);
                }
                TxStatus::Complete { cash_back: difference }
            }
        },
        Err(e) => {
            if let Err(e) = database::log_failed_transaction(tx_id, items) {
                eprintln!("Error logging failed transaction: {}", e);
            }

            TxStatus::Error {
                message: format!("Error querying items in database: {}", e),
            }
        }
    }
}

pub fn handle_credit(tx_id: u32, total: u32, items: Vec<TxEntry>, account_id: u32) -> TxStatus {
    //   - check if account has credit enabled
    //   - check if account is below credit limit
    //   - check if account allows overdraft

}