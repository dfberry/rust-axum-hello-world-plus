//use bson::Bson;
use mongodb::{bson::doc, /*bson::oid::ObjectId,*/ bson::Document, options::ClientOptions, options::FindOptions, Client, results::DeleteResult, results::UpdateResult};
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use futures::stream::TryStreamExt;
use chrono::{Utc};
use axum::{
    response::Json
};
use crate::types::ListExisting;
use crate::types::{ItemsParams, ItemNew, ItemExisting};
use futures::StreamExt;

const PARENT_COLLECTION: &str = "TodoList";
const CHILD_COLLECTION: &str = "TodoItem";

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
#[allow(non_snake_case)]
pub async fn update_list_by_id(id_str: String, list: Json<ListExisting>) -> Result<UpdateResult, Box<dyn Error>> {
    let collection = get_collection(PARENT_COLLECTION.to_string()).await?;
    let id = mongodb::bson::oid::ObjectId::from_str(&id_str).unwrap();
    let filter = doc! {
        "_id": id,
    };

    let utc_datetime_now:String = Utc::now().to_string();
    let update = doc! {
        "$set": {
            "name": list.name.clone().unwrap(),
            "updatedDate": &utc_datetime_now,
        },
    };
    let update_result = collection.update_one(filter, update, None).await?;
    Ok(update_result)
}

pub async fn get_items(id: String, params: ItemsParams) -> Result<Vec<Document>, Box<dyn Error>> {
    let collection = get_collection(CHILD_COLLECTION.to_string()).await?;

    let filter = doc! {
        "listid": mongodb::bson::oid::ObjectId::from_str(&id).unwrap(),
        "$text": {
            "$search": params.query
        }
    };

    let find_options = FindOptions::builder()
        .limit(params.top)
        .skip(params.skip)
        .build();

    let mut cursor = collection.find(Some(filter), find_options).await?;

    let mut docs = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        docs.push(result)
    }

    Ok(docs)
}
use bson::Bson;
pub async fn add_item(
    id: String,
    item_new: &Json<ItemNew>
) -> Result<String, Box<dyn Error>> {
    let collection = get_collection(CHILD_COLLECTION.to_string()).await?;
    
    let utc_datetime_now:String = Utc::now().to_string();
    let oid = mongodb::bson::oid::ObjectId::from_str(&id)?;


    let mut doc = Document::new();
    doc.insert("name", Bson::String(item_new.name.clone().unwrap_or(String::new())));
    doc.insert("listid", Bson::ObjectId(oid));
    doc.insert("state", "todo"); // default for new item
    doc.insert("name", Bson::String(item_new.description.clone().unwrap_or(String::new())));
    doc.insert("dueDate", Bson::String(utc_datetime_now.clone()));
    doc.insert("createdDate", Bson::String(utc_datetime_now.clone()));
    doc.insert("updatedDate", Bson::String(utc_datetime_now));

    let insert_one_result = collection.insert_one(doc, None).await?;

    Ok(insert_one_result.inserted_id.to_string())
}
pub async fn get_item_by_id(
    id: String
) -> Result<Vec<Document>, Box<dyn Error>> {
    let collection = get_collection(CHILD_COLLECTION.to_string()).await?;
    let oid = mongodb::bson::oid::ObjectId::from_str(&id)?;
    let filter = doc! {
        "_id": oid
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
#[allow(non_snake_case)]
pub async fn update_item_by_id(id_str: String, item: Json<ItemExisting>) -> Result<UpdateResult, Box<dyn Error>> {
    let collection = get_collection(CHILD_COLLECTION.to_string()).await?;
    let id = mongodb::bson::oid::ObjectId::from_str(&id_str).unwrap();
    let filter = doc! {
        "_id": id,
    };
    let state = item.state.clone().unwrap();    
    let utc_datetime_now:String = Utc::now().to_string();
    let update = doc! {
            "name": item.name.clone().unwrap(),
            "description": item.description.clone().unwrap(),
            "state": state.to_string(), 
            "dueDate": item.due_date.clone().unwrap(),
            "completedDate": item.completed_date.clone().unwrap(),
            "createdDate": item.created_date.clone().unwrap(),
            "updatedDate": &utc_datetime_now,
    };
    let update_result = collection.update_one(filter, update, None).await?;
    Ok(update_result)
}
pub async fn delete_item_by_id(id_str: String) -> Result<DeleteResult, Box<dyn Error>> {
    let collection = get_collection(CHILD_COLLECTION.to_string()).await?;
    let id = mongodb::bson::oid::ObjectId::from_str(&id_str).unwrap();
    let filter = doc! {
        "_id": id,
    };
    let delete_result = collection.delete_one(filter, None).await?;
    Ok(delete_result)
}
use crate::types::StateParams;
// get list of items by state
pub async fn get_items_by_state(
    state: String,
    params: StateParams
) -> Result<Vec<Document>, Box<dyn Error>> {
    let collection = get_collection(CHILD_COLLECTION.to_string()).await?;
    
    let filter = doc! {
        "state": state,
        "$text": {
            "$search": params.query
        }
    };

    let find_options = FindOptions::builder()
        .limit(params.top)
        .skip(params.skip)
        .build();

        let mut cursor = collection.find(Some(filter), find_options).await?;

        let mut docs = Vec::new();
    
        while let Some(result) = cursor.try_next().await? {
            docs.push(result)
        }
    
        Ok(docs)
}
pub async fn update_batch_state(
    state: String,
    params: StateParams,
    items: Json<Vec<ItemExisting>>
) -> Result<Vec<UpdateResult>, Box<dyn Error>> {
    let collection = get_collection(CHILD_COLLECTION.to_string()).await?;
    let utc_datetime_now:String = Utc::now().to_string();

    // vector of results
    let mut update_results = Vec::new();

    for item in items.iter() {
        let id_str = item.id.as_ref().ok_or("ID is missing")?;
        let id = mongodb::bson::oid::ObjectId::from_str(&id_str)?;

        let filter = doc! {
            "_id": id,
        };
        let update = doc! {
            "$set": {
                "state": state.to_string(),
                "updatedDate": &utc_datetime_now,
            },
        };
        // find and update 1
        let update_result = collection.update_one(filter, update, None).await?;
        
        // add to results
        update_results.push(update_result);
    }
    
    Ok(update_results)
}