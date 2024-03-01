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

pub async fn get_lists() -> Result<impl IntoResponse, StatusCode> {
    println!("get_lists");

    let data = database::get_list().await.unwrap();

    Ok(Json(data))
}
use crate::types::ListParams;
pub async fn get_lists_by_id(
    Query(params): Query<ListParams>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("get_lists_by_id: {}", id);
    println!("get_lists_by_id: {:?}", params);

    let data = database::get_list_by_id(id).await.unwrap();

    Ok(Json(data))
}
use crate::types::ListNew;
pub async fn add_list(list_new: Json<ListNew>
) -> Result<impl IntoResponse, StatusCode> {
    println!("add_list: {:?}", list_new);

    let name = &list_new.name;
    print!("name: {:?}", name);

    if name.is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    print!("name: {:?}", name.clone().unwrap());

    let id: String = database::add_list(name.clone().unwrap()).await.unwrap();

    Ok(Json(id))
}
pub async fn delete_list(    
    Query(params): Query<ListParams>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("delete_list: {}", id);
    println!("delete_list: {:?}", params);

    let delete_result = database::delete_list_by_id(id).await.unwrap();

    // if error return 500 else return success for delete
    Ok(Json(delete_result))
}
use crate::types::ListExisting;
pub async fn update_list_by_id(
    Query(params): Query<ListParams>,
    Path(id): Path<String>,
    list_existing: Json<ListExisting>
) -> Result<impl IntoResponse, StatusCode> {

    println!("update_list_by_id: {}", id);
    println!("update_list_by_id: {:?}", params);

    let update_result = database::update_list_by_id(id, list_existing).await.unwrap();

    // if error return 500 else return success for delete
    Ok(Json(update_result))
}
