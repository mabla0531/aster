use std::{collections::HashMap, sync::LazyLock};

use axum::{extract::Path, http::HeaderMap, response::Html, Json};
use model::{Account, Item, TransactionMethod, TransactionRequest, TransactionStatus};

use crate::{
    database,
    transaction::{handle_cash, handle_credit},
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
        (status = 201, description = "Transaction Response", body = TransactionStatus),
        (status = 500, description = "Transaction Error", body = String),
    ),
)]
pub async fn transaction(
    headers: HeaderMap,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionStatus>, String> {
    if check_auth(headers) == false {
        return Err("Unauthorized".to_string());
    }

    let TransactionRequest {
        tx_id,
        tender,
        items,
        method,
    } = payload;

    let items: HashMap<u32, u32> = items.iter().map(|item| (item.id, item.quantity)).collect();

    let result = match method {
        TransactionMethod::Cash => handle_cash(tx_id, tender, items).await,
        TransactionMethod::Credit { account_id } => handle_credit(tx_id, account_id, items).await,
    };

    result.map(|r| Json(r))
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
    if check_auth(headers) == false {
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
    if check_auth(headers) == false {
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
        (status = 201, description = "Account inserted", body = String),
        (status = 500, description = "Error inserting account", body = String),
    ),
)]
pub async fn insert_account(
    headers: HeaderMap,
    Json(payload): Json<Account>,
) -> Result<Json<String>, String> {
    if check_auth(headers) == false {
        return Err("Unauthorized".to_string());
    }

    match database::insert_account(payload).await {
        Ok(_) => Ok(Json("Account inserted".to_string())),
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
        (status = 200, description = "Pricebook Response", body = Vec<Item>),
        (status = 500, description = "Error querying items", body = String),
    ),
)]
pub async fn sync(headers: HeaderMap) -> Result<Json<Vec<Item>>, String> {
    if check_auth(headers) == false {
        return Err("Unauthorized".to_string());
    }

    database::get_all_items()
        .await
        .map(|items| Json(items))
        .map_err(|e| e.to_string())
}

pub fn check_auth(headers: HeaderMap) -> bool {
    match headers.get("x-auth-token") {
        Some(val) => {
            if val.to_str().unwrap_or_default() != *AUTH_KEY {
                false
            } else {
                true
            }
        }
        None => false,
    }
}
