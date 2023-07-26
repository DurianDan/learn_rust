use crate::models::Status;
use crate::db;

use deadpool_postgres::{Pool, Client};
use actix_web::{
    web,
    HttpResponse,
    Responder};

pub async fn status() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{user: "Huy".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    // web::Data<Pool> is for extracting the connection pool
    // from the data source config
    // (more details at the function `.data(poll)` in the `main.rs` file)
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
        // the `into()` function is called to turn the error into a response
    }
}