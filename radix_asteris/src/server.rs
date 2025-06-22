use std::{collections::HashMap, sync::LazyLock};

use axum::{Json, extract::Path, http::HeaderMap, response::Html};
use log::info;
use model::{
    Account, BalanceUpdate, SyncState, TransactionMethod, TransactionRequest, TransactionStatus,
};

use crate::{
    database,
    transaction::{calc_total, handle_cash, handle_credit},
};

pub static AUTH_KEY: LazyLock<&str> = LazyLock::new(|| include_str!("../../.env").trim());

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Default response", body = String),
    ),
)]
pub async fn default() -> Html<String> {
    Html("<!DOCTYPE html><html><head><title>get off my lawn</title></head><body><h1>This is not a website, please get out of my backend server</h1></body></html>".to_string())
}

#[utoipa::path(
    post,
    path = "/transaction",
    params(
        ("x-auth-token" = String, Header, description = "Authorization token"),
    ),
    responses(
        (status = 200, description = "Transaction Response", body = TransactionStatus),
        (status = 500, description = "Transaction Error", body = String),
    ),
)]
pub async fn transaction(
    headers: HeaderMap,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionStatus>, String> {
    info!("Transaction request: {:?}", payload);

    if !check_auth(headers) {
        return Err("Unauthorized".to_string());
    }

    let TransactionRequest {
        tx_id,
        items,
        method,
    } = payload;

    let items: HashMap<u32, u32> = items.iter().map(|item| (item.id, item.quantity)).collect();
    let total = calc_total(tx_id.clone(), items.clone()).await?;

    let result = match method {
        TransactionMethod::Cash { tender } => handle_cash(tx_id, tender, items, total).await,
        TransactionMethod::Credit { account_id } => {
            handle_credit(tx_id, account_id, items, total).await
        }
    };

    result.map(Json)
}

#[utoipa::path(
    get,
    path = "/accounts",
    params(
        ("x-auth-token" = String, Header, description = "Authorization token"),
    ),
    responses(
        (status = 200, description = "Accounts", body = Vec<Account>),
        (status = 500, description = "Error querying accounts", body = String),
    ),
)]
pub async fn get_accounts(headers: HeaderMap) -> Result<Json<Vec<Account>>, String> {
    info!("Get accounts request");

    if !check_auth(headers) {
        return Err("Unauthorized".to_string());
    }

    match database::get_all_accounts().await {
        Ok(accounts) => Ok(Json(accounts)),
        Err(e) => Err(e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/accounts/{account_id}",
    params(
        ("x-auth-token" = String, Header, description = "Authorization token"),
    ),
    responses(
        (status = 200, description = "Account", body = Account),
        (status = 500, description = "Error querying account", body = String),
    ),
)]
pub async fn get_account(
    headers: HeaderMap,
    Path(account_id): Path<u32>,
) -> Result<Json<Account>, String> {
    info!("Get account request for account_id: {}", account_id);

    if !check_auth(headers) {
        return Err("Unauthorized".to_string());
    }

    match database::get_account(account_id).await {
        Ok(account) => Ok(Json(account)),
        Err(e) => Err(e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/accounts/insert",
    params(
        ("x-auth-token" = String, Header, description = "Authorization token"),
    ),
    responses(
        (status = 200, description = "Account inserted", body = String),
        (status = 500, description = "Error inserting account", body = String),
    ),
)]
pub async fn insert_account(
    headers: HeaderMap,
    Json(payload): Json<Account>,
) -> Result<Json<String>, String> {
    info!("Insert account request: {:?}", payload);

    if !check_auth(headers) {
        return Err("Unauthorized".to_string());
    }

    match database::insert_account(payload).await {
        Ok(_) => Ok(Json("Account inserted".to_string())),
        Err(e) => Err(e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/accounts/balance",
    params(
        ("x-auth-token" = String, Header, description = "Authorization token"),
    ),
    responses(
        (status = 200, description = "Balance updated", body = String),
        (status = 500, description = "Error updating balance", body = String),
    ),
)]
pub async fn update_balance(
    headers: HeaderMap,
    Json(payload): Json<BalanceUpdate>,
) -> Result<Json<String>, String> {
    info!("Update balance request: {:?}", payload);

    if !check_auth(headers) {
        return Err("Unauthorized".to_string());
    }

    match database::update_balance(payload).await {
        Ok(_) => Ok(Json("Balance updated".to_string())),
        Err(e) => Err(e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/sync",
    params(
        ("x-auth-token" = String, Header, description = "Authorization token"),
    ),
    responses(
        (status = 200, description = "Pricebook Response", body = SyncState),
        (status = 500, description = "Error querying items", body = String),
    ),
)]
pub async fn sync(headers: HeaderMap) -> Result<Json<SyncState>, String> {
    info!("Sync request");

    if !check_auth(headers) {
        return Err("Unauthorized".to_string());
    }

    let pricebook = database::get_all_items().await.map_err(|e| e.to_string())?;
    let accounts = database::get_all_accounts()
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(SyncState {
        pricebook,
        accounts,
    }))
}

pub fn check_auth(headers: HeaderMap) -> bool {
    headers
        .get("x-auth-token")
        .map(|val| val.to_str().unwrap_or_default() == *AUTH_KEY)
        .unwrap_or(false)
}
