//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    extract::Path,
    response::Html, 
    response::IntoResponse, 
    http::StatusCode,
    routing::get, 
    Router, 
    response::Json
};

// JSON
use serde_json::json;
use serde::{Deserialize, Serialize};


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

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
async fn get_lists() -> impl IntoResponse {
    println!("get_lists");
    Html("<h1>lists</h1>")
}
async fn get_lists_by_id(Path(id): Path<i8>) -> Result<impl IntoResponse, StatusCode>{
    println!("get_lists_by_id: {}", id);

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
