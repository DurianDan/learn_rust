use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)] // means that the 'Status' struct is now an `impl` of `Serialize` which means it can be serialized by json()
pub struct Status {
    pub user: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
// the `PostgresMapper` macros generate more codes to import modules (like `pg_mapper`)
//
#[pg_mapper(table="todo_list")]
pub struct TodoList {
    pub id: i32, 
    pub title: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo_item")]
pub struct TodoItem {
    pub id: i32, 
    pub title: String,
    pub checked: bool, 
    pub list_id: i32 
}