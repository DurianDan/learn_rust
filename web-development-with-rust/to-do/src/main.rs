mod modules;
mod config;

use crate::modules::Status;
use actix_web::{HttpServer, HttpResponse, App, web, Responder};
use std::io;
use dotenv::dotenv;


async fn status() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{user: "Huy".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()>{

    dotenv().ok();
    
    let config = crate::config::Config::from_env().unwrap();

    println!("Starting the server at http://{}:{}", config.server.host, config.server.port);
    
    HttpServer::new(|| { 
        App::new()
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
