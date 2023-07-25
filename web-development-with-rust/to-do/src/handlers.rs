use crate::models::Status;
use actix_web::{
    HttpResponse,
    Responder};

pub async fn status() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{user: "Huy".to_string()})
}