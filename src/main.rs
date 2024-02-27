//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    response::Json,
    routing::get,
    Router,
};

// JSON
use serde_json::json;

// Serde - params
use serde::{de, Deserialize, Deserializer};

// To get Params
use std::{fmt, str::FromStr};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/lists", get(get_lists))
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
    // match opt.as_deref() {
    //     None | Some("") => Ok(None),
    //     Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    // }
    match opt.as_deref() {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => s.parse().map(Some).map_err(de::Error::custom),
        None => Ok(None),
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
async fn get_lists() -> impl IntoResponse {
    println!("get_lists");
    Html("<h1>lists</h1>")
}
// http://localhost:3000/lists/1?foo=1&bar=bar

// Use of Some for querystring
// get_lists_by_id: Params { foo: Some(1), bar: Some("bar") }
// {"age":43,"name":"John Doe","phones":["+44 1234567","+44 2345678"]}
// get_lists_by_id: 1
// get_lists_by_id: Params { foo: Some(1), bar: Some("") }
// {"age":43,"name":"John Doe","phones":["+44 1234567","+44 2345678"]}
// get_lists_by_id: 1
// get_lists_by_id: Params { foo: Some(1), bar: None }
// {"age":43,"name":"John Doe","phones":["+44 1234567","+44 2345678"]}

async fn get_lists_by_id(
    Query(params): Query<Params>,
    Path(id): Path<i8>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("get_lists_by_id: {}", id);
    println!("get_lists_by_id: {:?}", params);

    let data = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("{}", data.to_string());

    Ok(Json(data))
}
