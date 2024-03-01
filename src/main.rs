use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;
use std::ops::Deref;

// Serde - params
use serde::{de, Deserialize, Deserializer};

// To get Params
// use std::error::Error;
use std::{fmt, str::FromStr};

pub mod database;
pub mod route_items;
pub mod route_lists;
pub mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler_root))
        .route("/lists", get(route_lists::get_lists))
        .route("/lists", post(route_lists::add_list))
        .route("/lists/:id", delete(route_lists::delete_list))
        .route("/lists/:id", put(route_lists::update_list_by_id))
        .route("/lists/:id", get(route_lists::get_lists_by_id))
        .route("/lists/:id/items", get(route_items::get_items))
        .route("/lists/:id/items", post(route_items::add_item))
        .route("/lists/:id/items/:itemId", get(route_items::get_item_by_id))
        .route(
            "/lists/:id/items/:itemId",
            put(route_items::update_item_by_id),
        )
        .route(
            "/lists/:id/items/:itemId",
            delete(route_items::delete_item_by_id),
        )
        .route("/state/:state", get(route_items::get_items_by_state))
        .route("/state/:state", put(route_items::update_items_state));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler_root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
