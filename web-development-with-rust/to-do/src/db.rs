use crate::models::TodoList;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error>{
    let statement = client
        .prepare("SELECT * FROM todo_list")
        .await
        .unwrap();
    let todo = client.query(&statement, &[]) // &[] is for parameters
        .await
        .expect("Error getting todo lists")
        .iter() // turn the query results into a iterable of `TokioPostgresRow`
        .map(|row| TodoList::from_row_ref(row).unwrap())
        // turn TokioPostgresRow into the struct TodoList
        // each `item` in the todo vector is now can be serialized into json
        .collect::<Vec<TodoList>>(); // collect the results of the map function
        // the `collect` method convert an `iterable` into another.
        // The TokioPostgresRow can be turned into the TodoList
        //   because some `impl`s has been added to the struct `TodoList`
        //   through the attribute macro `pg_mapper` (details at `models.rs`)
    Ok(todo)
    
}