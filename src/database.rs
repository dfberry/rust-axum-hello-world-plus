//use bson::Bson;
use mongodb::{bson::doc, /*bson::oid::ObjectId,*/ bson::Document, options::ClientOptions, Client};
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
pub async fn get_async_hello() -> String {
    // simulate some async operation
    sleep(Duration::from_secs(2)).await;
    "Hello from Rust!".to_string()
}

// use mongodb::{Client, options::ClientOptions, options::FindOptions};
// use mongodb::bson::{doc, Document};

#[derive(Serialize, Debug)]
struct Err {}
impl From<mongodb::error::Error> for Err {
    fn from(_error: mongodb::error::Error) -> Self {
        Err {}
    }
}

#[allow(dead_code)]
type DatabaseResult<T> = std::result::Result<T, Err>;

pub async fn get_list() -> Result<Vec<Document>, Box<dyn Error>> {
    let connection_string =
        std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name =
        std::env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME must be set.");
    let collection_name =
        std::env::var("MONGODB_COLLECTION_NAME").expect("MONGODB_COLLECTION_NAME must be set.");

    let client_options = ClientOptions::parse(&connection_string).await?;
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
pub async fn get_list_by_id(id_str: String) -> Result<Vec<Document>, Box<dyn Error>> {
    let connection_string =
        std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name =
        std::env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME must be set.");
    let collection_name =
        std::env::var("MONGODB_COLLECTION_NAME").expect("MONGODB_COLLECTION_NAME must be set.");

    let client_options = ClientOptions::parse(&connection_string).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(&database_name);

    for collection_name in database.list_collection_names(None).await? {
        println!("{}", collection_name);
        //let collection = database.collection::<Document>(&collection_name);
    }

    let nontyped_collection = database.collection::<Document>(&collection_name);

    use futures::stream::TryStreamExt;

    let filter = doc! {
        "_id": mongodb::bson::oid::ObjectId::from_str(&id_str).unwrap()
    };
    let mut cursor = nontyped_collection
        .find(Some(filter), None)
        .await
        .expect("error occured");
    let mut docs = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        docs.push(result)
    }

    Ok(docs)
}
