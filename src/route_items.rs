use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    response::Json,
    routing::{get, post, delete, put},
    Router,
};
use dotenv::dotenv;
use std::ops::Deref;

// Serde - params
use serde::{de, Deserialize, Deserializer};

// To get Params
// use std::error::Error;
use std::{fmt, str::FromStr};

use crate::database;
use crate::types;
use crate::types::ItemsParams;
use crate::types::ItemNew;

pub async fn get_items(
    Query(params): Query<ItemsParams>, 
    Path(id): Path<String>
) -> Result<impl IntoResponse, StatusCode> {
    println!("get_items: {}", id);
    println!("get_items: {:?}", params);

    let data = database::get_items(id, params).await.unwrap();

    Ok(Json(data))
}

pub async fn add_item(
    Query(params): Query<ItemsParams>,
    Path(id): Path<String>,
    item_new: Json<ItemNew>
) -> Result<impl IntoResponse, StatusCode> {
    println!("add_item: {:?}", item_new);

    let id: String = database::add_item(id, &item_new).await.unwrap();

    Ok(Json(id))
}
pub async fn get_item_by_id(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("get_items_by_id: {}", id);

    let data = database::get_item_by_id(id).await.unwrap();

    Ok(Json(data))
}
pub async fn update_item_by_id(
    Path(id): Path<String>,
    item_existing: Json<types::ItemExisting>
) -> Result<impl IntoResponse, StatusCode> {
    println!("update_item_by_id: {}", id);
    println!("update_item_by_id: {:?}", item_existing);

    let update_result = database::update_item_by_id(id, item_existing).await.unwrap();

    Ok(Json(update_result))
}
pub async fn delete_item_by_id(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("delete_item_by_id: {}", id);

    let delete_result = database::delete_item_by_id(id).await.unwrap();

    Ok(Json(delete_result))
}
use crate::types::StateParams;
pub async fn get_items_by_state(
    Path(state): Path<String>,
    Query(params): Query<StateParams>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("get_items_by_state: {}", state);
    println!("get_items: {:?}", params);

    let data = database::get_items_by_state(state, params).await.unwrap();

    Ok(Json(data))
}
pub async fn update_items_state(
    Path(state): Path<String>,
    Query(params): Query<StateParams>,
    // add param for batch of items in body
    items_existing: Json<Vec<types::ItemExisting>>
) -> Result<impl IntoResponse, StatusCode> {
    println!("update_items_state: {}", state);
    println!("update_items_state: {:?}", params);

    let update_result = database::update_batch_state(state, params, items_existing).await.unwrap();

    Ok(Json(update_result))
}
