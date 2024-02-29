use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use dotenv::dotenv;

// Serde - params
use serde::{de, Deserialize, Deserializer};

// To get Params
// use std::error::Error;
use std::{fmt, str::FromStr};
pub mod database;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler_root))
        .route("/lists", get(get_lists))
        .route("/lists", post(add_list))
        .route("/lists/:id", delete(delete_list))
        .route("/lists/:id", get(get_lists_by_id));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    foo: Option<i32>,
    bar: Option<String>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;

    match opt.as_deref() {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => s.parse().map(Some).map_err(de::Error::custom),
        None => Ok(None),
    }
}

async fn handler_root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


async fn get_lists() -> Result<impl IntoResponse, StatusCode> {
    println!("get_lists");

    let data = database::get_list().await.unwrap();

    Ok(Json(data))
}

async fn get_lists_by_id(
    Query(params): Query<Params>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("get_lists_by_id: {}", id);
    println!("get_lists_by_id: {:?}", params);

    let data = database::get_list_by_id(id).await.unwrap();

    Ok(Json(data))
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ListNew {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    name: Option<String>,
}
async fn add_list(list_new: Json<ListNew>
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
async fn delete_list(    
    Query(params): Query<Params>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("delete_list: {}", id);
    println!("delete_list: {:?}", params);

    let delete_result = database::delete_list_by_id(id).await.unwrap();

    // if error return 500 else return success for delete
    Ok(Json(delete_result))
}