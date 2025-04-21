use axum::{response::Html, Json};

use crate::{model::{TransactionMethod, TransactionRequestBody, TransactionResponseBody}, transaction::{handle_cash, handle_credit}};

pub async fn default() -> Html<String> {
    Html("<!DOCTYPE html><html><head><title>get off my lawn</title></head><body><h1>This is not a website, please get out of my backend server</h1></body></html>".to_string())
}

pub async fn transaction(Json(payload): Json<TransactionRequestBody>) -> Json<TransactionResponseBody> {
    
    let TransactionRequestBody {
        tx_id,
        total,
        items,
        method,
    } = payload;

    let result = match method {
        TransactionMethod::Cash => handle_cash(tx_id, total, items),
        TransactionMethod::Credit { account_id } => handle_credit(tx_id, total, items, account_id),
    };

    Json(result.into())
}