//use bson::Bson;
use mongodb::{bson::doc, /*bson::oid::ObjectId,*/ bson::Document, options::ClientOptions, Client, results::DeleteResult};
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use futures::stream::TryStreamExt;
use chrono::{Utc};


const PARENT_COLLECTION: &str = "TodoList";

fn get_env_var() -> (String, String, String) {
    let connection_string =
        std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name =
        std::env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME must be set.");
    let collection_name =
        std::env::var("MONGODB_COLLECTION_NAME").expect("MONGODB_COLLECTION_NAME must be set.");
    (connection_string, database_name, collection_name)
}
type DatabaseResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn get_database () -> Result<mongodb::Database, Box<dyn Error>> {
    let (connection_string, database_name, _collection_name) = get_env_var();
    let client_options = ClientOptions::parse(&connection_string).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(&database_name);
    Ok(database)
}
pub async fn get_collections () -> DatabaseResult<Vec<String>> {
    let database = get_database().await?;

    let mut collections = Vec::new();
    for collection_name in database.list_collection_names(None).await? {
        collections.push(collection_name);
    }
    Ok(collections)
}
async fn get_collection (collection_name: String)-> Result<mongodb::Collection<Document>, Box<dyn Error>> {

    let database = get_database().await?;
    let collection = database.collection(&collection_name);
    Ok(collection)
}

pub async fn get_async_hello() -> String {
    // simulate some async operation
    sleep(Duration::from_secs(2)).await;
    "Hello from Rust!".to_string()
}

#[derive(Serialize, Debug)]
struct Err {}
impl From<mongodb::error::Error> for Err {
    fn from(_error: mongodb::error::Error) -> Self {
        Err {}
    }
}

pub async fn get_list() -> Result<Vec<Document>, Box<dyn Error>> {

    let collection = get_collection(PARENT_COLLECTION.to_string()).await?;

    let mut cursor = collection
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
    let collection = get_collection(PARENT_COLLECTION.to_string()).await?;
    let filter = doc! {
        "_id": mongodb::bson::oid::ObjectId::from_str(&id_str).unwrap()
    };
    let mut cursor = collection
        .find(Some(filter), None)
        .await
        .expect("error occured");
    let mut docs = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        docs.push(result)
    }

    Ok(docs)
}
pub async fn add_list(name: String) -> Result<String, Box<dyn Error>> {
    let collection = get_collection(PARENT_COLLECTION.to_string()).await?;
    
    let utc_datetime_now:String = Utc::now().to_string();

    let doc = doc! {
        "name": name,
        "createdDate": &utc_datetime_now,
        "updatedDate": &utc_datetime_now,
    };

    let insert_one_result = collection.insert_one(doc, None).await?;

    Ok(insert_one_result.inserted_id.to_string())
}
pub async fn delete_list_by_id(id_str: String) -> Result<DeleteResult, Box<dyn Error>> {
    let collection = get_collection(PARENT_COLLECTION.to_string()).await?;
    let id = mongodb::bson::oid::ObjectId::from_str(&id_str).unwrap();
    let filter = doc! {
        "_id": id,
    };
    let delete_result = collection.delete_one(filter, None).await?;
    Ok(delete_result)
}