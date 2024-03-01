use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ListParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    foo: Option<i32>,
    bar: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ListNew {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ListExisting {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub name: Option<String>,
    pub _id: Option<String>,
    pub createdDate: Option<String>,
    pub updatedDate: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub enum ItemState {
    Todo,
    InProgress,
    Done,
}

// add trait to convert to string for ItemState
impl ToString for ItemState {
    fn to_string(&self) -> String {
        match self {
            ItemState::Todo => "todo".to_string(),
            ItemState::InProgress => "inprogress".to_string(),
            ItemState::Done => "done".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ListItemParams{
        listId: Option<String>,
        itemId: Option<String>,
        state: ItemState
    }
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ItemsParams{
    pub query: Option<String>,
    pub skip: Option<u64>,
    pub top: Option<i64>,

}
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ItemNew {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub name: Option<String>,
    pub state: Option<ItemState>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub completed_date: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ItemExisting {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub id: Option<String>,
    pub name: Option<String>,
    pub state: Option<ItemState>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub completed_date: Option<String>,
    pub created_date: Option<String>,
    pub updated_date: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct StateParams{
    pub query: Option<String>,
    pub skip: Option<u64>,
    pub top: Option<i64>,

}