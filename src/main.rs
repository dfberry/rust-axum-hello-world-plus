use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    response::Json,
    routing::get,
    Router,
};
use dotenv::dotenv;

// JSON
use serde_json::json;

// Serde - params
use serde::{de, Deserialize, Deserializer};

// To get Params
use std::error::Error;
use std::{fmt, str::FromStr};
//-----------------------------------------------------------------------------------------------------------------------
use mongodb::{bson::doc, bson::Document, options::ClientOptions, Client};

// use mongodb::{Client, options::ClientOptions, options::FindOptions};
// use mongodb::bson::{doc, Document};
use serde::Serialize;
#[derive(Serialize, Deserialize, Debug)]
struct Err {}
impl From<mongodb::error::Error> for Err {
    fn from(_error: mongodb::error::Error) -> Self {
        Err {}
    }
}

#[allow(dead_code)]
type DatabaseResult<T> = std::result::Result<T, Err>;

async fn get_list() -> Result<Vec<Document>, Box<dyn Error>> {
    let connection_string = std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = std::env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME must be set.");
    let collection_name = std::env::var("MONGODB_COLLECTION_NAME").expect("MONGODB_COLLECTION_NAME must be set.");

    let client_options = ClientOptions::parse(
        &connection_string,
        )
        .await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(&database_name);

    for collection_name in database.list_collection_names(None).await? {
        println!("{}", collection_name);
        //let collection = database.collection::<Document>(&collection_name);
    }

    let nontyped_collection = database.collection::<Document>(&collection_name);

    use futures::stream::TryStreamExt;

    let mut cursor = nontyped_collection
        .find(None, None)
        .await
        .expect("error occured");
    let mut docs = Vec::new();

    while let Some(result) = cursor.try_next().await.unwrap() {
        docs.push(result)
    }

    Ok(docs)
}

//-----------------------------------------------------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler_root))
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

async fn handler_root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

/*
[
    {
        "name": "My List",
        "createdDate": "2024-02-28T14:04:09.466Z",
        "updatedDate": "2024-02-28T14:04:09.466Z",
        "id": "65df3d5934761793df5fbe46"
    }
]
*/
// struct ListStruct {
//     name: String,
//     created_date: String,
//     updated_date: String,
//     _id: String, // underscore in cosmos, not underscore returned to client
// }

async fn get_lists() -> Result<impl IntoResponse, StatusCode> {
    println!("get_lists");

    let data = get_list().await.unwrap();

    Ok(Json(data))
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

/*
add list item
https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists/65df3d5934761793df5fbe46/items
payload: {"name":"Get eggs and milk","listId":"65df3d5934761793df5fbe46","state":"todo"}
201 Created
*/

/*
get item to edit
https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists/65df3d5934761793df5fbe46/items/65df3e7134761793df5fbe4d
{
    "listId": "65df3d5934761793df5fbe46",
    "name": "Get eggs and milk",
    "state": "todo",
    "createdDate": "2024-02-28T14:08:49.279Z",
    "updatedDate": "2024-02-28T14:08:49.279Z",
    "id": "65df3e7134761793df5fbe4d"
}
 */

/* edit item
PUT https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists/65df3d5934761793df5fbe46/items/65df3e7134761793df5fbe4d
payload:{"id":"65df3e7134761793df5fbe4d","listId":"65df3d5934761793df5fbe46","name":"Get eggs and milk 2","description":"  2","dueDate":"2024-02-15T08:00:00.000Z","state":"inprogress"}

response:

{
    "listId": "65df3d5934761793df5fbe46",
    "name": "Get eggs and milk 2",
    "state": "inprogress",
    "createdDate": "2024-02-28T14:08:49.279Z",
    "updatedDate": "2024-02-28T14:12:14.343Z",
    "dueDate": "2024-02-15T08:00:00.000Z",
    "description": "  2",
    "id": "65df3e7134761793df5fbe4d"
}

200

 */

/* Add new list
POST https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists
payload: {"name":"My second list"}

response:

{
    "name": "My second list",
    "createdDate": "2024-02-28T14:13:38.400Z",
    "updatedDate": "2024-02-28T14:13:38.400Z",
    "id": "65df3f9234761793df5fbe53"
}

201 created


*/

/*
get list by id
GET https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists/65df3f9234761793df5fbe53

response;

{
    "name": "My second list",
    "createdDate": "2024-02-28T14:13:38.400Z",
    "updatedDate": "2024-02-28T14:13:38.400Z",
    "id": "65df3f9234761793df5fbe53"
}

200 OK

*/

/* get all items on list
GET
https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists/65df3f9234761793df5fbe53/items
200 OK
response:

[
    {
        "listId": "65df3d5934761793df5fbe46",
        "name": "Get eggs and milk 2",
        "state": "inprogress",
        "createdDate": "2024-02-28T14:08:49.279Z",
        "updatedDate": "2024-02-28T14:12:14.343Z",
        "dueDate": "2024-02-15T08:00:00.000Z",
        "description": "  2",
        "id": "65df3e7134761793df5fbe4d"
    },
    {
        "listId": "65df3d5934761793df5fbe46",
        "name": "Walk the dog",
        "state": "todo",
        "createdDate": "2024-02-28T14:10:43.455Z",
        "updatedDate": "2024-02-28T14:10:43.455Z",
        "id": "65df3ee334761793df5fbe4f"
    }
]


*/

/* Delete an item
DELETE https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists/65df3d5934761793df5fbe46/items/65df3ee334761793df5fbe4f
204 No Content
*/

/* Delete a list
DELETE
https://ca-api-hsl6vouqg5mme.blackpond-29967867.eastus2.azurecontainerapps.io/lists/65df3f9234761793df5fbe53
204 No Content
*/
